//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-ses

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
pub fn serialize_clone_receipt_rule_set_response(
    result: &CloneReceiptRuleSetResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CloneReceiptRuleSetResult>{inner_xml}</CloneReceiptRuleSetResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CloneReceiptRuleSetResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CloneReceiptRuleSetResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_configuration_set_response(
    result: &CreateConfigurationSetResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateConfigurationSetResult>{inner_xml}</CreateConfigurationSetResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateConfigurationSetResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateConfigurationSetResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_configuration_set_event_destination_response(
    result: &CreateConfigurationSetEventDestinationResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<CreateConfigurationSetEventDestinationResult>{inner_xml}</CreateConfigurationSetEventDestinationResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateConfigurationSetEventDestinationResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateConfigurationSetEventDestinationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_configuration_set_tracking_options_response(
    result: &CreateConfigurationSetTrackingOptionsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<CreateConfigurationSetTrackingOptionsResult>{inner_xml}</CreateConfigurationSetTrackingOptionsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateConfigurationSetTrackingOptionsResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateConfigurationSetTrackingOptionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_create_custom_verification_email_template_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateCustomVerificationEmailTemplateResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateCustomVerificationEmailTemplateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_receipt_filter_response(
    result: &CreateReceiptFilterResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateReceiptFilterResult>{inner_xml}</CreateReceiptFilterResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateReceiptFilterResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateReceiptFilterResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_receipt_rule_response(result: &CreateReceiptRuleResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateReceiptRuleResult>{inner_xml}</CreateReceiptRuleResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateReceiptRuleResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateReceiptRuleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_receipt_rule_set_response(
    result: &CreateReceiptRuleSetResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateReceiptRuleSetResult>{inner_xml}</CreateReceiptRuleSetResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateReceiptRuleSetResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateReceiptRuleSetResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_template_response(result: &CreateTemplateResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateTemplateResult>{inner_xml}</CreateTemplateResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateTemplateResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateTemplateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_configuration_set_response(
    result: &DeleteConfigurationSetResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DeleteConfigurationSetResult>{inner_xml}</DeleteConfigurationSetResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteConfigurationSetResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteConfigurationSetResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_configuration_set_event_destination_response(
    result: &DeleteConfigurationSetEventDestinationResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DeleteConfigurationSetEventDestinationResult>{inner_xml}</DeleteConfigurationSetEventDestinationResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteConfigurationSetEventDestinationResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteConfigurationSetEventDestinationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_configuration_set_tracking_options_response(
    result: &DeleteConfigurationSetTrackingOptionsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DeleteConfigurationSetTrackingOptionsResult>{inner_xml}</DeleteConfigurationSetTrackingOptionsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteConfigurationSetTrackingOptionsResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteConfigurationSetTrackingOptionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_custom_verification_email_template_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteCustomVerificationEmailTemplateResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteCustomVerificationEmailTemplateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_identity_response(result: &DeleteIdentityResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteIdentityResult>{inner_xml}</DeleteIdentityResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteIdentityResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteIdentityResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_identity_policy_response(
    result: &DeleteIdentityPolicyResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DeleteIdentityPolicyResult>{inner_xml}</DeleteIdentityPolicyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteIdentityPolicyResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteIdentityPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_receipt_filter_response(
    result: &DeleteReceiptFilterResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteReceiptFilterResult>{inner_xml}</DeleteReceiptFilterResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteReceiptFilterResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteReceiptFilterResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_receipt_rule_response(result: &DeleteReceiptRuleResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteReceiptRuleResult>{inner_xml}</DeleteReceiptRuleResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteReceiptRuleResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteReceiptRuleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_receipt_rule_set_response(
    result: &DeleteReceiptRuleSetResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DeleteReceiptRuleSetResult>{inner_xml}</DeleteReceiptRuleSetResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteReceiptRuleSetResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteReceiptRuleSetResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_template_response(result: &DeleteTemplateResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteTemplateResult>{inner_xml}</DeleteTemplateResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteTemplateResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteTemplateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_verified_email_address_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteVerifiedEmailAddressResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteVerifiedEmailAddressResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_active_receipt_rule_set_response(
    result: &DescribeActiveReceiptRuleSetResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeActiveReceiptRuleSetResult>{inner_xml}</DescribeActiveReceiptRuleSetResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeActiveReceiptRuleSetResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeActiveReceiptRuleSetResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_configuration_set_response(
    result: &DescribeConfigurationSetResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeConfigurationSetResult>{inner_xml}</DescribeConfigurationSetResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeConfigurationSetResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeConfigurationSetResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_receipt_rule_response(
    result: &DescribeReceiptRuleResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeReceiptRuleResult>{inner_xml}</DescribeReceiptRuleResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeReceiptRuleResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeReceiptRuleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_receipt_rule_set_response(
    result: &DescribeReceiptRuleSetResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeReceiptRuleSetResult>{inner_xml}</DescribeReceiptRuleSetResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeReceiptRuleSetResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeReceiptRuleSetResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_account_sending_enabled_response(
    result: &GetAccountSendingEnabledResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<GetAccountSendingEnabledResult>{inner_xml}</GetAccountSendingEnabledResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetAccountSendingEnabledResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetAccountSendingEnabledResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_custom_verification_email_template_response(
    result: &GetCustomVerificationEmailTemplateResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<GetCustomVerificationEmailTemplateResult>{inner_xml}</GetCustomVerificationEmailTemplateResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetCustomVerificationEmailTemplateResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetCustomVerificationEmailTemplateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_identity_dkim_attributes_response(
    result: &GetIdentityDkimAttributesResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<GetIdentityDkimAttributesResult>{inner_xml}</GetIdentityDkimAttributesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetIdentityDkimAttributesResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetIdentityDkimAttributesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_identity_mail_from_domain_attributes_response(
    result: &GetIdentityMailFromDomainAttributesResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<GetIdentityMailFromDomainAttributesResult>{inner_xml}</GetIdentityMailFromDomainAttributesResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetIdentityMailFromDomainAttributesResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetIdentityMailFromDomainAttributesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_identity_notification_attributes_response(
    result: &GetIdentityNotificationAttributesResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<GetIdentityNotificationAttributesResult>{inner_xml}</GetIdentityNotificationAttributesResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetIdentityNotificationAttributesResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetIdentityNotificationAttributesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_identity_policies_response(
    result: &GetIdentityPoliciesResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetIdentityPoliciesResult>{inner_xml}</GetIdentityPoliciesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetIdentityPoliciesResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetIdentityPoliciesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_identity_verification_attributes_response(
    result: &GetIdentityVerificationAttributesResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<GetIdentityVerificationAttributesResult>{inner_xml}</GetIdentityVerificationAttributesResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetIdentityVerificationAttributesResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetIdentityVerificationAttributesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_send_quota_response(result: &GetSendQuotaResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetSendQuotaResult>{inner_xml}</GetSendQuotaResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetSendQuotaResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetSendQuotaResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_send_statistics_response(result: &GetSendStatisticsResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetSendStatisticsResult>{inner_xml}</GetSendStatisticsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetSendStatisticsResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetSendStatisticsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_template_response(result: &GetTemplateResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetTemplateResult>{inner_xml}</GetTemplateResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetTemplateResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetTemplateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_configuration_sets_response(
    result: &ListConfigurationSetsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListConfigurationSetsResult>{inner_xml}</ListConfigurationSetsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListConfigurationSetsResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListConfigurationSetsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_custom_verification_email_templates_response(
    result: &ListCustomVerificationEmailTemplatesResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ListCustomVerificationEmailTemplatesResult>{inner_xml}</ListCustomVerificationEmailTemplatesResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListCustomVerificationEmailTemplatesResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListCustomVerificationEmailTemplatesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_identities_response(result: &ListIdentitiesResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListIdentitiesResult>{inner_xml}</ListIdentitiesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListIdentitiesResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListIdentitiesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_identity_policies_response(
    result: &ListIdentityPoliciesResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListIdentityPoliciesResult>{inner_xml}</ListIdentityPoliciesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListIdentityPoliciesResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListIdentityPoliciesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_receipt_filters_response(
    result: &ListReceiptFiltersResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListReceiptFiltersResult>{inner_xml}</ListReceiptFiltersResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListReceiptFiltersResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListReceiptFiltersResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_receipt_rule_sets_response(
    result: &ListReceiptRuleSetsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListReceiptRuleSetsResult>{inner_xml}</ListReceiptRuleSetsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListReceiptRuleSetsResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListReceiptRuleSetsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_templates_response(result: &ListTemplatesResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListTemplatesResult>{inner_xml}</ListTemplatesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListTemplatesResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListTemplatesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_verified_email_addresses_response(
    result: &ListVerifiedEmailAddressesResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListVerifiedEmailAddressesResult>{inner_xml}</ListVerifiedEmailAddressesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListVerifiedEmailAddressesResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListVerifiedEmailAddressesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_put_configuration_set_delivery_options_response(
    result: &PutConfigurationSetDeliveryOptionsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<PutConfigurationSetDeliveryOptionsResult>{inner_xml}</PutConfigurationSetDeliveryOptionsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PutConfigurationSetDeliveryOptionsResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PutConfigurationSetDeliveryOptionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_put_identity_policy_response(result: &PutIdentityPolicyResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<PutIdentityPolicyResult>{inner_xml}</PutIdentityPolicyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PutIdentityPolicyResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PutIdentityPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_reorder_receipt_rule_set_response(
    result: &ReorderReceiptRuleSetResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ReorderReceiptRuleSetResult>{inner_xml}</ReorderReceiptRuleSetResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ReorderReceiptRuleSetResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ReorderReceiptRuleSetResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_send_bounce_response(result: &SendBounceResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<SendBounceResult>{inner_xml}</SendBounceResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SendBounceResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SendBounceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_send_bulk_templated_email_response(
    result: &SendBulkTemplatedEmailResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<SendBulkTemplatedEmailResult>{inner_xml}</SendBulkTemplatedEmailResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SendBulkTemplatedEmailResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SendBulkTemplatedEmailResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_send_custom_verification_email_response(
    result: &SendCustomVerificationEmailResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<SendCustomVerificationEmailResult>{inner_xml}</SendCustomVerificationEmailResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SendCustomVerificationEmailResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SendCustomVerificationEmailResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_send_email_response(result: &SendEmailResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<SendEmailResult>{inner_xml}</SendEmailResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SendEmailResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SendEmailResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_send_raw_email_response(result: &SendRawEmailResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<SendRawEmailResult>{inner_xml}</SendRawEmailResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SendRawEmailResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SendRawEmailResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_send_templated_email_response(
    result: &SendTemplatedEmailResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<SendTemplatedEmailResult>{inner_xml}</SendTemplatedEmailResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SendTemplatedEmailResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SendTemplatedEmailResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_set_active_receipt_rule_set_response(
    result: &SetActiveReceiptRuleSetResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<SetActiveReceiptRuleSetResult>{inner_xml}</SetActiveReceiptRuleSetResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetActiveReceiptRuleSetResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetActiveReceiptRuleSetResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_set_identity_dkim_enabled_response(
    result: &SetIdentityDkimEnabledResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<SetIdentityDkimEnabledResult>{inner_xml}</SetIdentityDkimEnabledResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetIdentityDkimEnabledResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetIdentityDkimEnabledResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_set_identity_feedback_forwarding_enabled_response(
    result: &SetIdentityFeedbackForwardingEnabledResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<SetIdentityFeedbackForwardingEnabledResult>{inner_xml}</SetIdentityFeedbackForwardingEnabledResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetIdentityFeedbackForwardingEnabledResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetIdentityFeedbackForwardingEnabledResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_set_identity_headers_in_notifications_enabled_response(
    result: &SetIdentityHeadersInNotificationsEnabledResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<SetIdentityHeadersInNotificationsEnabledResult>{inner_xml}</SetIdentityHeadersInNotificationsEnabledResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetIdentityHeadersInNotificationsEnabledResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetIdentityHeadersInNotificationsEnabledResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_set_identity_mail_from_domain_response(
    result: &SetIdentityMailFromDomainResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<SetIdentityMailFromDomainResult>{inner_xml}</SetIdentityMailFromDomainResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetIdentityMailFromDomainResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetIdentityMailFromDomainResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_set_identity_notification_topic_response(
    result: &SetIdentityNotificationTopicResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<SetIdentityNotificationTopicResult>{inner_xml}</SetIdentityNotificationTopicResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetIdentityNotificationTopicResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetIdentityNotificationTopicResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_set_receipt_rule_position_response(
    result: &SetReceiptRulePositionResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<SetReceiptRulePositionResult>{inner_xml}</SetReceiptRulePositionResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetReceiptRulePositionResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetReceiptRulePositionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_test_render_template_response(
    result: &TestRenderTemplateResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<TestRenderTemplateResult>{inner_xml}</TestRenderTemplateResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<TestRenderTemplateResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</TestRenderTemplateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_update_account_sending_enabled_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateAccountSendingEnabledResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateAccountSendingEnabledResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_update_configuration_set_event_destination_response(
    result: &UpdateConfigurationSetEventDestinationResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<UpdateConfigurationSetEventDestinationResult>{inner_xml}</UpdateConfigurationSetEventDestinationResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateConfigurationSetEventDestinationResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateConfigurationSetEventDestinationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_update_configuration_set_reputation_metrics_enabled_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateConfigurationSetReputationMetricsEnabledResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateConfigurationSetReputationMetricsEnabledResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_update_configuration_set_sending_enabled_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateConfigurationSetSendingEnabledResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateConfigurationSetSendingEnabledResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_update_configuration_set_tracking_options_response(
    result: &UpdateConfigurationSetTrackingOptionsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<UpdateConfigurationSetTrackingOptionsResult>{inner_xml}</UpdateConfigurationSetTrackingOptionsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateConfigurationSetTrackingOptionsResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateConfigurationSetTrackingOptionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_update_custom_verification_email_template_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateCustomVerificationEmailTemplateResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateCustomVerificationEmailTemplateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_update_receipt_rule_response(result: &UpdateReceiptRuleResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<UpdateReceiptRuleResult>{inner_xml}</UpdateReceiptRuleResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateReceiptRuleResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateReceiptRuleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_update_template_response(result: &UpdateTemplateResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<UpdateTemplateResult>{inner_xml}</UpdateTemplateResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateTemplateResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateTemplateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_verify_domain_dkim_response(result: &VerifyDomainDkimResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<VerifyDomainDkimResult>{inner_xml}</VerifyDomainDkimResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<VerifyDomainDkimResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</VerifyDomainDkimResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_verify_domain_identity_response(
    result: &VerifyDomainIdentityResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<VerifyDomainIdentityResult>{inner_xml}</VerifyDomainIdentityResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<VerifyDomainIdentityResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</VerifyDomainIdentityResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_verify_email_address_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<VerifyEmailAddressResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</VerifyEmailAddressResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_verify_email_identity_response(
    result: &VerifyEmailIdentityResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<VerifyEmailIdentityResult>{inner_xml}</VerifyEmailIdentityResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<VerifyEmailIdentityResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</VerifyEmailIdentityResponse>"#
    );
    MockResponse::xml(200, xml)
}

fn deserialize_stop_action_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<StopAction>, String> {
    let mut item = StopAction::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Scope")) {
        item.scope = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.TopicArn")) {
        item.topic_arn = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_bounced_recipient_info_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<BouncedRecipientInfo>, String> {
    let mut item = BouncedRecipientInfo::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.BounceType")) {
        item.bounce_type = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Recipient")) {
        item.recipient = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.RecipientArn")) {
        item.recipient_arn = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_body_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<Body>, String> {
    let mut item = Body::default();
    let mut found = false;
    Ok(if found { Some(item) } else { None })
}

fn deserialize_configuration_set_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ConfigurationSet>, String> {
    let mut item = ConfigurationSet::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Name")) {
        item.name = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_receipt_action_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ReceiptAction>, String> {
    let mut item = ReceiptAction::default();
    let mut found = false;
    Ok(if found { Some(item) } else { None })
}

fn deserialize_connect_action_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ConnectAction>, String> {
    let mut item = ConnectAction::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.IAMRoleARN")) {
        item.i_a_m_role_a_r_n = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.InstanceARN")) {
        item.instance_a_r_n = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_message_tag_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<MessageTag>, String> {
    let mut item = MessageTag::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Name")) {
        item.name = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Value")) {
        item.value = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_raw_message_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<RawMessage>, String> {
    let mut item = RawMessage::default();
    let mut found = false;
    Ok(if found { Some(item) } else { None })
}

fn deserialize_workmail_action_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<WorkmailAction>, String> {
    let mut item = WorkmailAction::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.OrganizationArn")) {
        item.organization_arn = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.TopicArn")) {
        item.topic_arn = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_receipt_filter_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ReceiptFilter>, String> {
    let mut item = ReceiptFilter::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Name")) {
        item.name = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_template_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<Template>, String> {
    let mut item = Template::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.HtmlPart")) {
        item.html_part = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.SubjectPart")) {
        item.subject_part = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.TemplateName")) {
        item.template_name = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.TextPart")) {
        item.text_part = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_bounce_action_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<BounceAction>, String> {
    let mut item = BounceAction::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Message")) {
        item.message = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Sender")) {
        item.sender = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.SmtpReplyCode")) {
        item.smtp_reply_code = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.StatusCode")) {
        item.status_code = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.TopicArn")) {
        item.topic_arn = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_lambda_action_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<LambdaAction>, String> {
    let mut item = LambdaAction::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.FunctionArn")) {
        item.function_arn = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.InvocationType")) {
        item.invocation_type = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.TopicArn")) {
        item.topic_arn = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_message_dsn_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<MessageDsn>, String> {
    let mut item = MessageDsn::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.ArrivalDate")) {
        item.arrival_date = Some(value.to_string());
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.ExtensionFields");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_extension_field_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.extension_fields = Some(ExtensionFieldList { items: sub_items });
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.ReportingMta")) {
        item.reporting_mta = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_cloud_watch_destination_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<CloudWatchDestination>, String> {
    let mut item = CloudWatchDestination::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.DimensionConfigurations");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_cloud_watch_dimension_configuration_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.dimension_configurations = CloudWatchDimensionConfigurations { items: sub_items };
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_cloud_watch_dimension_configuration_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<CloudWatchDimensionConfiguration>, String> {
    let mut item = CloudWatchDimensionConfiguration::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.DefaultDimensionValue")) {
        item.default_dimension_value = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.DimensionName")) {
        item.dimension_name = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.DimensionValueSource")) {
        item.dimension_value_source = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_tracking_options_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<TrackingOptions>, String> {
    let mut item = TrackingOptions::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.CustomRedirectDomain")) {
        item.custom_redirect_domain = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_message_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<Message>, String> {
    let mut item = Message::default();
    let mut found = false;
    Ok(if found { Some(item) } else { None })
}

fn deserialize_receipt_ip_filter_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ReceiptIpFilter>, String> {
    let mut item = ReceiptIpFilter::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Cidr")) {
        item.cidr = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Policy")) {
        item.policy = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_extension_field_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ExtensionField>, String> {
    let mut item = ExtensionField::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Name")) {
        item.name = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Value")) {
        item.value = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_bulk_email_destination_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<BulkEmailDestination>, String> {
    let mut item = BulkEmailDestination::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.ReplacementTags");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_message_tag_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.replacement_tags = Some(MessageTagList { items: sub_items });
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.ReplacementTemplateData")) {
        item.replacement_template_data = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_s_n_s_action_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<SNSAction>, String> {
    let mut item = SNSAction::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Encoding")) {
        item.encoding = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.TopicArn")) {
        item.topic_arn = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_destination_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<Destination>, String> {
    let mut item = Destination::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.BccAddresses");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.bcc_addresses = Some(AddressList { items: sub_items });
            found = true;
        }
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.CcAddresses");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.cc_addresses = Some(AddressList { items: sub_items });
            found = true;
        }
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.ToAddresses");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.to_addresses = Some(AddressList { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_content_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<Content>, String> {
    let mut item = Content::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Charset")) {
        item.charset = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Data")) {
        item.data = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_event_destination_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<EventDestination>, String> {
    let mut item = EventDestination::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Enabled")) {
        item.enabled = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Enabled: {e}"))?,
        );
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.MatchingEventTypes");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.matching_event_types = EventTypes { items: sub_items };
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.Name")) {
        item.name = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_add_header_action_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<AddHeaderAction>, String> {
    let mut item = AddHeaderAction::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.HeaderName")) {
        item.header_name = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.HeaderValue")) {
        item.header_value = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_delivery_options_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<DeliveryOptions>, String> {
    let mut item = DeliveryOptions::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.TlsPolicy")) {
        item.tls_policy = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_receipt_rule_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ReceiptRule>, String> {
    let mut item = ReceiptRule::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Actions");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_receipt_action_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.actions = Some(ReceiptActionsList { items: sub_items });
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.Enabled")) {
        item.enabled = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Enabled: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Name")) {
        item.name = value.to_string();
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Recipients");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.recipients = Some(RecipientsList { items: sub_items });
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.ScanEnabled")) {
        item.scan_enabled = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ScanEnabled: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.TlsPolicy")) {
        item.tls_policy = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_s_n_s_destination_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<SNSDestination>, String> {
    let mut item = SNSDestination::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.TopicARN")) {
        item.topic_a_r_n = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_kinesis_firehose_destination_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<KinesisFirehoseDestination>, String> {
    let mut item = KinesisFirehoseDestination::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.DeliveryStreamARN")) {
        item.delivery_stream_a_r_n = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.IAMRoleARN")) {
        item.i_a_m_role_a_r_n = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_s3_action_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<S3Action>, String> {
    let mut item = S3Action::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.BucketName")) {
        item.bucket_name = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.IamRoleArn")) {
        item.iam_role_arn = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.KmsKeyArn")) {
        item.kms_key_arn = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ObjectKeyPrefix")) {
        item.object_key_prefix = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.TopicArn")) {
        item.topic_arn = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_recipient_dsn_fields_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<RecipientDsnFields>, String> {
    let mut item = RecipientDsnFields::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Action")) {
        item.action = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.DiagnosticCode")) {
        item.diagnostic_code = Some(value.to_string());
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.ExtensionFields");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_extension_field_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.extension_fields = Some(ExtensionFieldList { items: sub_items });
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.FinalRecipient")) {
        item.final_recipient = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.LastAttemptDate")) {
        item.last_attempt_date = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.RemoteMta")) {
        item.remote_mta = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Status")) {
        item.status = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

/// Deserialize awsQuery request for CloneReceiptRuleSet.
pub fn deserialize_clone_receipt_rule_set_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CloneReceiptRuleSetRequest, String> {
    let mut input = CloneReceiptRuleSetRequest::default();
    if let Some(value) = params.get("OriginalRuleSetName") {
        input.original_rule_set_name = value.to_string();
    }
    if let Some(value) = params.get("RuleSetName") {
        input.rule_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateConfigurationSet.
pub fn deserialize_create_configuration_set_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateConfigurationSetRequest, String> {
    let mut input = CreateConfigurationSetRequest::default();
    if let Some(val) = deserialize_configuration_set_from_query(params, "ConfigurationSet")? {
        input.configuration_set = val;
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateConfigurationSetEventDestination.
pub fn deserialize_create_configuration_set_event_destination_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateConfigurationSetEventDestinationRequest, String> {
    let mut input = CreateConfigurationSetEventDestinationRequest::default();
    if let Some(value) = params.get("ConfigurationSetName") {
        input.configuration_set_name = value.to_string();
    }
    if let Some(val) = deserialize_event_destination_from_query(params, "EventDestination")? {
        input.event_destination = val;
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateConfigurationSetTrackingOptions.
pub fn deserialize_create_configuration_set_tracking_options_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateConfigurationSetTrackingOptionsRequest, String> {
    let mut input = CreateConfigurationSetTrackingOptionsRequest::default();
    if let Some(value) = params.get("ConfigurationSetName") {
        input.configuration_set_name = value.to_string();
    }
    if let Some(val) = deserialize_tracking_options_from_query(params, "TrackingOptions")? {
        input.tracking_options = val;
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateCustomVerificationEmailTemplate.
pub fn deserialize_create_custom_verification_email_template_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateCustomVerificationEmailTemplateRequest, String> {
    let mut input = CreateCustomVerificationEmailTemplateRequest::default();
    if let Some(value) = params.get("FailureRedirectionURL") {
        input.failure_redirection_u_r_l = value.to_string();
    }
    if let Some(value) = params.get("FromEmailAddress") {
        input.from_email_address = value.to_string();
    }
    if let Some(value) = params.get("SuccessRedirectionURL") {
        input.success_redirection_u_r_l = value.to_string();
    }
    if let Some(value) = params.get("TemplateContent") {
        input.template_content = value.to_string();
    }
    if let Some(value) = params.get("TemplateName") {
        input.template_name = value.to_string();
    }
    if let Some(value) = params.get("TemplateSubject") {
        input.template_subject = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateReceiptFilter.
pub fn deserialize_create_receipt_filter_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateReceiptFilterRequest, String> {
    let mut input = CreateReceiptFilterRequest::default();
    if let Some(val) = deserialize_receipt_filter_from_query(params, "Filter")? {
        input.filter = val;
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateReceiptRule.
pub fn deserialize_create_receipt_rule_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateReceiptRuleRequest, String> {
    let mut input = CreateReceiptRuleRequest::default();
    if let Some(value) = params.get("After") {
        input.after = Some(value.to_string());
    }
    if let Some(val) = deserialize_receipt_rule_from_query(params, "Rule")? {
        input.rule = val;
    }
    if let Some(value) = params.get("RuleSetName") {
        input.rule_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateReceiptRuleSet.
pub fn deserialize_create_receipt_rule_set_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateReceiptRuleSetRequest, String> {
    let mut input = CreateReceiptRuleSetRequest::default();
    if let Some(value) = params.get("RuleSetName") {
        input.rule_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateTemplate.
pub fn deserialize_create_template_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateTemplateRequest, String> {
    let mut input = CreateTemplateRequest::default();
    if let Some(val) = deserialize_template_from_query(params, "Template")? {
        input.template = val;
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteConfigurationSet.
pub fn deserialize_delete_configuration_set_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteConfigurationSetRequest, String> {
    let mut input = DeleteConfigurationSetRequest::default();
    if let Some(value) = params.get("ConfigurationSetName") {
        input.configuration_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteConfigurationSetEventDestination.
pub fn deserialize_delete_configuration_set_event_destination_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteConfigurationSetEventDestinationRequest, String> {
    let mut input = DeleteConfigurationSetEventDestinationRequest::default();
    if let Some(value) = params.get("ConfigurationSetName") {
        input.configuration_set_name = value.to_string();
    }
    if let Some(value) = params.get("EventDestinationName") {
        input.event_destination_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteConfigurationSetTrackingOptions.
pub fn deserialize_delete_configuration_set_tracking_options_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteConfigurationSetTrackingOptionsRequest, String> {
    let mut input = DeleteConfigurationSetTrackingOptionsRequest::default();
    if let Some(value) = params.get("ConfigurationSetName") {
        input.configuration_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteCustomVerificationEmailTemplate.
pub fn deserialize_delete_custom_verification_email_template_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteCustomVerificationEmailTemplateRequest, String> {
    let mut input = DeleteCustomVerificationEmailTemplateRequest::default();
    if let Some(value) = params.get("TemplateName") {
        input.template_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteIdentity.
pub fn deserialize_delete_identity_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteIdentityRequest, String> {
    let mut input = DeleteIdentityRequest::default();
    if let Some(value) = params.get("Identity") {
        input.identity = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteIdentityPolicy.
pub fn deserialize_delete_identity_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteIdentityPolicyRequest, String> {
    let mut input = DeleteIdentityPolicyRequest::default();
    if let Some(value) = params.get("Identity") {
        input.identity = value.to_string();
    }
    if let Some(value) = params.get("PolicyName") {
        input.policy_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteReceiptFilter.
pub fn deserialize_delete_receipt_filter_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteReceiptFilterRequest, String> {
    let mut input = DeleteReceiptFilterRequest::default();
    if let Some(value) = params.get("FilterName") {
        input.filter_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteReceiptRule.
pub fn deserialize_delete_receipt_rule_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteReceiptRuleRequest, String> {
    let mut input = DeleteReceiptRuleRequest::default();
    if let Some(value) = params.get("RuleName") {
        input.rule_name = value.to_string();
    }
    if let Some(value) = params.get("RuleSetName") {
        input.rule_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteReceiptRuleSet.
pub fn deserialize_delete_receipt_rule_set_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteReceiptRuleSetRequest, String> {
    let mut input = DeleteReceiptRuleSetRequest::default();
    if let Some(value) = params.get("RuleSetName") {
        input.rule_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteTemplate.
pub fn deserialize_delete_template_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteTemplateRequest, String> {
    let mut input = DeleteTemplateRequest::default();
    if let Some(value) = params.get("TemplateName") {
        input.template_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteVerifiedEmailAddress.
pub fn deserialize_delete_verified_email_address_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteVerifiedEmailAddressRequest, String> {
    let mut input = DeleteVerifiedEmailAddressRequest::default();
    if let Some(value) = params.get("EmailAddress") {
        input.email_address = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeActiveReceiptRuleSet.
pub fn deserialize_describe_active_receipt_rule_set_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeActiveReceiptRuleSetRequest, String> {
    let input = DescribeActiveReceiptRuleSetRequest {};
    Ok(input)
}

/// Deserialize awsQuery request for DescribeConfigurationSet.
pub fn deserialize_describe_configuration_set_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeConfigurationSetRequest, String> {
    let mut input = DescribeConfigurationSetRequest::default();
    {
        let mut items = Vec::new();
        let list_prefix = "ConfigurationSetAttributeNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.configuration_set_attribute_names = Some(ConfigurationSetAttributeList { items });
        }
    }
    if let Some(value) = params.get("ConfigurationSetName") {
        input.configuration_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeReceiptRule.
pub fn deserialize_describe_receipt_rule_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeReceiptRuleRequest, String> {
    let mut input = DescribeReceiptRuleRequest::default();
    if let Some(value) = params.get("RuleName") {
        input.rule_name = value.to_string();
    }
    if let Some(value) = params.get("RuleSetName") {
        input.rule_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeReceiptRuleSet.
pub fn deserialize_describe_receipt_rule_set_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeReceiptRuleSetRequest, String> {
    let mut input = DescribeReceiptRuleSetRequest::default();
    if let Some(value) = params.get("RuleSetName") {
        input.rule_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetCustomVerificationEmailTemplate.
pub fn deserialize_get_custom_verification_email_template_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetCustomVerificationEmailTemplateRequest, String> {
    let mut input = GetCustomVerificationEmailTemplateRequest::default();
    if let Some(value) = params.get("TemplateName") {
        input.template_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetIdentityDkimAttributes.
pub fn deserialize_get_identity_dkim_attributes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetIdentityDkimAttributesRequest, String> {
    let mut input = GetIdentityDkimAttributesRequest::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Identities".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.identities = IdentityList { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetIdentityMailFromDomainAttributes.
pub fn deserialize_get_identity_mail_from_domain_attributes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetIdentityMailFromDomainAttributesRequest, String> {
    let mut input = GetIdentityMailFromDomainAttributesRequest::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Identities".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.identities = IdentityList { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetIdentityNotificationAttributes.
pub fn deserialize_get_identity_notification_attributes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetIdentityNotificationAttributesRequest, String> {
    let mut input = GetIdentityNotificationAttributesRequest::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Identities".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.identities = IdentityList { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetIdentityPolicies.
pub fn deserialize_get_identity_policies_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetIdentityPoliciesRequest, String> {
    let mut input = GetIdentityPoliciesRequest::default();
    if let Some(value) = params.get("Identity") {
        input.identity = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "PolicyNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.policy_names = PolicyNameList { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetIdentityVerificationAttributes.
pub fn deserialize_get_identity_verification_attributes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetIdentityVerificationAttributesRequest, String> {
    let mut input = GetIdentityVerificationAttributesRequest::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Identities".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.identities = IdentityList { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetTemplate.
pub fn deserialize_get_template_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetTemplateRequest, String> {
    let mut input = GetTemplateRequest::default();
    if let Some(value) = params.get("TemplateName") {
        input.template_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListConfigurationSets.
pub fn deserialize_list_configuration_sets_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListConfigurationSetsRequest, String> {
    let mut input = ListConfigurationSetsRequest::default();
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListCustomVerificationEmailTemplates.
pub fn deserialize_list_custom_verification_email_templates_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListCustomVerificationEmailTemplatesRequest, String> {
    let mut input = ListCustomVerificationEmailTemplatesRequest::default();
    if let Some(value) = params.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxResults: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListIdentities.
pub fn deserialize_list_identities_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListIdentitiesRequest, String> {
    let mut input = ListIdentitiesRequest::default();
    if let Some(value) = params.get("IdentityType") {
        input.identity_type = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListIdentityPolicies.
pub fn deserialize_list_identity_policies_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListIdentityPoliciesRequest, String> {
    let mut input = ListIdentityPoliciesRequest::default();
    if let Some(value) = params.get("Identity") {
        input.identity = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListReceiptFilters.
pub fn deserialize_list_receipt_filters_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListReceiptFiltersRequest, String> {
    let input = ListReceiptFiltersRequest {};
    Ok(input)
}

/// Deserialize awsQuery request for ListReceiptRuleSets.
pub fn deserialize_list_receipt_rule_sets_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListReceiptRuleSetsRequest, String> {
    let mut input = ListReceiptRuleSetsRequest::default();
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListTemplates.
pub fn deserialize_list_templates_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListTemplatesRequest, String> {
    let mut input = ListTemplatesRequest::default();
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for PutConfigurationSetDeliveryOptions.
pub fn deserialize_put_configuration_set_delivery_options_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<PutConfigurationSetDeliveryOptionsRequest, String> {
    let mut input = PutConfigurationSetDeliveryOptionsRequest::default();
    if let Some(value) = params.get("ConfigurationSetName") {
        input.configuration_set_name = value.to_string();
    }
    if let Some(val) = deserialize_delivery_options_from_query(params, "DeliveryOptions")? {
        input.delivery_options = Some(val);
    }
    Ok(input)
}

/// Deserialize awsQuery request for PutIdentityPolicy.
pub fn deserialize_put_identity_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<PutIdentityPolicyRequest, String> {
    let mut input = PutIdentityPolicyRequest::default();
    if let Some(value) = params.get("Identity") {
        input.identity = value.to_string();
    }
    if let Some(value) = params.get("Policy") {
        input.policy = value.to_string();
    }
    if let Some(value) = params.get("PolicyName") {
        input.policy_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ReorderReceiptRuleSet.
pub fn deserialize_reorder_receipt_rule_set_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ReorderReceiptRuleSetRequest, String> {
    let mut input = ReorderReceiptRuleSetRequest::default();
    {
        let mut items = Vec::new();
        let list_prefix = "RuleNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.rule_names = ReceiptRuleNamesList { items };
        }
    }
    if let Some(value) = params.get("RuleSetName") {
        input.rule_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for SendBounce.
pub fn deserialize_send_bounce_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SendBounceRequest, String> {
    let mut input = SendBounceRequest::default();
    if let Some(value) = params.get("BounceSender") {
        input.bounce_sender = value.to_string();
    }
    if let Some(value) = params.get("BounceSenderArn") {
        input.bounce_sender_arn = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "BouncedRecipientInfoList".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_bounced_recipient_info_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.bounced_recipient_info_list = BouncedRecipientInfoList { items };
        }
    }
    if let Some(value) = params.get("Explanation") {
        input.explanation = Some(value.to_string());
    }
    if let Some(val) = deserialize_message_dsn_from_query(params, "MessageDsn")? {
        input.message_dsn = Some(val);
    }
    if let Some(value) = params.get("OriginalMessageId") {
        input.original_message_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for SendBulkTemplatedEmail.
pub fn deserialize_send_bulk_templated_email_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SendBulkTemplatedEmailRequest, String> {
    let mut input = SendBulkTemplatedEmailRequest::default();
    if let Some(value) = params.get("ConfigurationSetName") {
        input.configuration_set_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "DefaultTags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_message_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.default_tags = Some(MessageTagList { items });
        }
    }
    if let Some(value) = params.get("DefaultTemplateData") {
        input.default_template_data = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Destinations".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_bulk_email_destination_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.destinations = BulkEmailDestinationList { items };
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ReplyToAddresses".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.reply_to_addresses = Some(AddressList { items });
        }
    }
    if let Some(value) = params.get("ReturnPath") {
        input.return_path = Some(value.to_string());
    }
    if let Some(value) = params.get("ReturnPathArn") {
        input.return_path_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("Source") {
        input.source = value.to_string();
    }
    if let Some(value) = params.get("SourceArn") {
        input.source_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("Template") {
        input.template = value.to_string();
    }
    if let Some(value) = params.get("TemplateArn") {
        input.template_arn = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for SendCustomVerificationEmail.
pub fn deserialize_send_custom_verification_email_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SendCustomVerificationEmailRequest, String> {
    let mut input = SendCustomVerificationEmailRequest::default();
    if let Some(value) = params.get("ConfigurationSetName") {
        input.configuration_set_name = Some(value.to_string());
    }
    if let Some(value) = params.get("EmailAddress") {
        input.email_address = value.to_string();
    }
    if let Some(value) = params.get("TemplateName") {
        input.template_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for SendEmail.
pub fn deserialize_send_email_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SendEmailRequest, String> {
    let mut input = SendEmailRequest::default();
    if let Some(value) = params.get("ConfigurationSetName") {
        input.configuration_set_name = Some(value.to_string());
    }
    if let Some(val) = deserialize_destination_from_query(params, "Destination")? {
        input.destination = val;
    }
    if let Some(val) = deserialize_message_from_query(params, "Message")? {
        input.message = val;
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ReplyToAddresses".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.reply_to_addresses = Some(AddressList { items });
        }
    }
    if let Some(value) = params.get("ReturnPath") {
        input.return_path = Some(value.to_string());
    }
    if let Some(value) = params.get("ReturnPathArn") {
        input.return_path_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("Source") {
        input.source = value.to_string();
    }
    if let Some(value) = params.get("SourceArn") {
        input.source_arn = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_message_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(MessageTagList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for SendRawEmail.
pub fn deserialize_send_raw_email_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SendRawEmailRequest, String> {
    let mut input = SendRawEmailRequest::default();
    if let Some(value) = params.get("ConfigurationSetName") {
        input.configuration_set_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Destinations".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.destinations = Some(AddressList { items });
        }
    }
    if let Some(value) = params.get("FromArn") {
        input.from_arn = Some(value.to_string());
    }
    if let Some(val) = deserialize_raw_message_from_query(params, "RawMessage")? {
        input.raw_message = val;
    }
    if let Some(value) = params.get("ReturnPathArn") {
        input.return_path_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("Source") {
        input.source = Some(value.to_string());
    }
    if let Some(value) = params.get("SourceArn") {
        input.source_arn = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_message_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(MessageTagList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for SendTemplatedEmail.
pub fn deserialize_send_templated_email_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SendTemplatedEmailRequest, String> {
    let mut input = SendTemplatedEmailRequest::default();
    if let Some(value) = params.get("ConfigurationSetName") {
        input.configuration_set_name = Some(value.to_string());
    }
    if let Some(val) = deserialize_destination_from_query(params, "Destination")? {
        input.destination = val;
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ReplyToAddresses".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.reply_to_addresses = Some(AddressList { items });
        }
    }
    if let Some(value) = params.get("ReturnPath") {
        input.return_path = Some(value.to_string());
    }
    if let Some(value) = params.get("ReturnPathArn") {
        input.return_path_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("Source") {
        input.source = value.to_string();
    }
    if let Some(value) = params.get("SourceArn") {
        input.source_arn = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_message_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(MessageTagList { items });
        }
    }
    if let Some(value) = params.get("Template") {
        input.template = value.to_string();
    }
    if let Some(value) = params.get("TemplateArn") {
        input.template_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("TemplateData") {
        input.template_data = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for SetActiveReceiptRuleSet.
pub fn deserialize_set_active_receipt_rule_set_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetActiveReceiptRuleSetRequest, String> {
    let mut input = SetActiveReceiptRuleSetRequest::default();
    if let Some(value) = params.get("RuleSetName") {
        input.rule_set_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for SetIdentityDkimEnabled.
pub fn deserialize_set_identity_dkim_enabled_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetIdentityDkimEnabledRequest, String> {
    let mut input = SetIdentityDkimEnabledRequest::default();
    if let Some(value) = params.get("DkimEnabled") {
        input.dkim_enabled = value
            .parse::<bool>()
            .map_err(|e| format!("failed to parse DkimEnabled: {e}"))?;
    }
    if let Some(value) = params.get("Identity") {
        input.identity = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for SetIdentityFeedbackForwardingEnabled.
pub fn deserialize_set_identity_feedback_forwarding_enabled_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetIdentityFeedbackForwardingEnabledRequest, String> {
    let mut input = SetIdentityFeedbackForwardingEnabledRequest::default();
    if let Some(value) = params.get("ForwardingEnabled") {
        input.forwarding_enabled = value
            .parse::<bool>()
            .map_err(|e| format!("failed to parse ForwardingEnabled: {e}"))?;
    }
    if let Some(value) = params.get("Identity") {
        input.identity = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for SetIdentityHeadersInNotificationsEnabled.
pub fn deserialize_set_identity_headers_in_notifications_enabled_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetIdentityHeadersInNotificationsEnabledRequest, String> {
    let mut input = SetIdentityHeadersInNotificationsEnabledRequest::default();
    if let Some(value) = params.get("Enabled") {
        input.enabled = value
            .parse::<bool>()
            .map_err(|e| format!("failed to parse Enabled: {e}"))?;
    }
    if let Some(value) = params.get("Identity") {
        input.identity = value.to_string();
    }
    if let Some(value) = params.get("NotificationType") {
        input.notification_type = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for SetIdentityMailFromDomain.
pub fn deserialize_set_identity_mail_from_domain_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetIdentityMailFromDomainRequest, String> {
    let mut input = SetIdentityMailFromDomainRequest::default();
    if let Some(value) = params.get("BehaviorOnMXFailure") {
        input.behavior_on_m_x_failure = Some(value.to_string());
    }
    if let Some(value) = params.get("Identity") {
        input.identity = value.to_string();
    }
    if let Some(value) = params.get("MailFromDomain") {
        input.mail_from_domain = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for SetIdentityNotificationTopic.
pub fn deserialize_set_identity_notification_topic_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetIdentityNotificationTopicRequest, String> {
    let mut input = SetIdentityNotificationTopicRequest::default();
    if let Some(value) = params.get("Identity") {
        input.identity = value.to_string();
    }
    if let Some(value) = params.get("NotificationType") {
        input.notification_type = value.to_string();
    }
    if let Some(value) = params.get("SnsTopic") {
        input.sns_topic = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for SetReceiptRulePosition.
pub fn deserialize_set_receipt_rule_position_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetReceiptRulePositionRequest, String> {
    let mut input = SetReceiptRulePositionRequest::default();
    if let Some(value) = params.get("After") {
        input.after = Some(value.to_string());
    }
    if let Some(value) = params.get("RuleName") {
        input.rule_name = value.to_string();
    }
    if let Some(value) = params.get("RuleSetName") {
        input.rule_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for TestRenderTemplate.
pub fn deserialize_test_render_template_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<TestRenderTemplateRequest, String> {
    let mut input = TestRenderTemplateRequest::default();
    if let Some(value) = params.get("TemplateData") {
        input.template_data = value.to_string();
    }
    if let Some(value) = params.get("TemplateName") {
        input.template_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateAccountSendingEnabled.
pub fn deserialize_update_account_sending_enabled_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateAccountSendingEnabledRequest, String> {
    let mut input = UpdateAccountSendingEnabledRequest::default();
    if let Some(value) = params.get("Enabled") {
        input.enabled = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Enabled: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateConfigurationSetEventDestination.
pub fn deserialize_update_configuration_set_event_destination_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateConfigurationSetEventDestinationRequest, String> {
    let mut input = UpdateConfigurationSetEventDestinationRequest::default();
    if let Some(value) = params.get("ConfigurationSetName") {
        input.configuration_set_name = value.to_string();
    }
    if let Some(val) = deserialize_event_destination_from_query(params, "EventDestination")? {
        input.event_destination = val;
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateConfigurationSetReputationMetricsEnabled.
pub fn deserialize_update_configuration_set_reputation_metrics_enabled_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateConfigurationSetReputationMetricsEnabledRequest, String> {
    let mut input = UpdateConfigurationSetReputationMetricsEnabledRequest::default();
    if let Some(value) = params.get("ConfigurationSetName") {
        input.configuration_set_name = value.to_string();
    }
    if let Some(value) = params.get("Enabled") {
        input.enabled = value
            .parse::<bool>()
            .map_err(|e| format!("failed to parse Enabled: {e}"))?;
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateConfigurationSetSendingEnabled.
pub fn deserialize_update_configuration_set_sending_enabled_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateConfigurationSetSendingEnabledRequest, String> {
    let mut input = UpdateConfigurationSetSendingEnabledRequest::default();
    if let Some(value) = params.get("ConfigurationSetName") {
        input.configuration_set_name = value.to_string();
    }
    if let Some(value) = params.get("Enabled") {
        input.enabled = value
            .parse::<bool>()
            .map_err(|e| format!("failed to parse Enabled: {e}"))?;
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateConfigurationSetTrackingOptions.
pub fn deserialize_update_configuration_set_tracking_options_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateConfigurationSetTrackingOptionsRequest, String> {
    let mut input = UpdateConfigurationSetTrackingOptionsRequest::default();
    if let Some(value) = params.get("ConfigurationSetName") {
        input.configuration_set_name = value.to_string();
    }
    if let Some(val) = deserialize_tracking_options_from_query(params, "TrackingOptions")? {
        input.tracking_options = val;
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateCustomVerificationEmailTemplate.
pub fn deserialize_update_custom_verification_email_template_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateCustomVerificationEmailTemplateRequest, String> {
    let mut input = UpdateCustomVerificationEmailTemplateRequest::default();
    if let Some(value) = params.get("FailureRedirectionURL") {
        input.failure_redirection_u_r_l = Some(value.to_string());
    }
    if let Some(value) = params.get("FromEmailAddress") {
        input.from_email_address = Some(value.to_string());
    }
    if let Some(value) = params.get("SuccessRedirectionURL") {
        input.success_redirection_u_r_l = Some(value.to_string());
    }
    if let Some(value) = params.get("TemplateContent") {
        input.template_content = Some(value.to_string());
    }
    if let Some(value) = params.get("TemplateName") {
        input.template_name = value.to_string();
    }
    if let Some(value) = params.get("TemplateSubject") {
        input.template_subject = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateReceiptRule.
pub fn deserialize_update_receipt_rule_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateReceiptRuleRequest, String> {
    let mut input = UpdateReceiptRuleRequest::default();
    if let Some(val) = deserialize_receipt_rule_from_query(params, "Rule")? {
        input.rule = val;
    }
    if let Some(value) = params.get("RuleSetName") {
        input.rule_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateTemplate.
pub fn deserialize_update_template_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateTemplateRequest, String> {
    let mut input = UpdateTemplateRequest::default();
    if let Some(val) = deserialize_template_from_query(params, "Template")? {
        input.template = val;
    }
    Ok(input)
}

/// Deserialize awsQuery request for VerifyDomainDkim.
pub fn deserialize_verify_domain_dkim_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<VerifyDomainDkimRequest, String> {
    let mut input = VerifyDomainDkimRequest::default();
    if let Some(value) = params.get("Domain") {
        input.domain = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for VerifyDomainIdentity.
pub fn deserialize_verify_domain_identity_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<VerifyDomainIdentityRequest, String> {
    let mut input = VerifyDomainIdentityRequest::default();
    if let Some(value) = params.get("Domain") {
        input.domain = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for VerifyEmailAddress.
pub fn deserialize_verify_email_address_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<VerifyEmailAddressRequest, String> {
    let mut input = VerifyEmailAddressRequest::default();
    if let Some(value) = params.get("EmailAddress") {
        input.email_address = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for VerifyEmailIdentity.
pub fn deserialize_verify_email_identity_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<VerifyEmailIdentityRequest, String> {
    let mut input = VerifyEmailIdentityRequest::default();
    if let Some(value) = params.get("EmailAddress") {
        input.email_address = value.to_string();
    }
    Ok(input)
}
