//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-sns

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
pub fn serialize_add_permission_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AddPermissionResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AddPermissionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_check_if_phone_number_is_opted_out_response(
    result: &CheckIfPhoneNumberIsOptedOutResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<CheckIfPhoneNumberIsOptedOutResult>{inner_xml}</CheckIfPhoneNumberIsOptedOutResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CheckIfPhoneNumberIsOptedOutResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CheckIfPhoneNumberIsOptedOutResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_confirm_subscription_response(
    result: &ConfirmSubscriptionResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ConfirmSubscriptionResult>{inner_xml}</ConfirmSubscriptionResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ConfirmSubscriptionResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ConfirmSubscriptionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_platform_application_response(
    result: &CreatePlatformApplicationResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreatePlatformApplicationResult>{inner_xml}</CreatePlatformApplicationResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreatePlatformApplicationResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreatePlatformApplicationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_platform_endpoint_response(
    result: &CreateEndpointResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreatePlatformEndpointResult>{inner_xml}</CreatePlatformEndpointResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreatePlatformEndpointResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreatePlatformEndpointResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_s_m_s_sandbox_phone_number_response(
    result: &CreateSMSSandboxPhoneNumberResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<CreateSMSSandboxPhoneNumberResult>{inner_xml}</CreateSMSSandboxPhoneNumberResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateSMSSandboxPhoneNumberResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateSMSSandboxPhoneNumberResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_topic_response(result: &CreateTopicResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateTopicResult>{inner_xml}</CreateTopicResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateTopicResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateTopicResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_endpoint_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteEndpointResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteEndpointResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_platform_application_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeletePlatformApplicationResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeletePlatformApplicationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_s_m_s_sandbox_phone_number_response(
    result: &DeleteSMSSandboxPhoneNumberResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DeleteSMSSandboxPhoneNumberResult>{inner_xml}</DeleteSMSSandboxPhoneNumberResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteSMSSandboxPhoneNumberResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteSMSSandboxPhoneNumberResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_topic_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteTopicResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteTopicResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_data_protection_policy_response(
    result: &GetDataProtectionPolicyResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<GetDataProtectionPolicyResult>{inner_xml}</GetDataProtectionPolicyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetDataProtectionPolicyResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetDataProtectionPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_endpoint_attributes_response(
    result: &GetEndpointAttributesResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<GetEndpointAttributesResult>{inner_xml}</GetEndpointAttributesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetEndpointAttributesResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetEndpointAttributesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_platform_application_attributes_response(
    result: &GetPlatformApplicationAttributesResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<GetPlatformApplicationAttributesResult>{inner_xml}</GetPlatformApplicationAttributesResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetPlatformApplicationAttributesResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetPlatformApplicationAttributesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_s_m_s_attributes_response(result: &GetSMSAttributesResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetSMSAttributesResult>{inner_xml}</GetSMSAttributesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetSMSAttributesResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetSMSAttributesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_s_m_s_sandbox_account_status_response(
    result: &GetSMSSandboxAccountStatusResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<GetSMSSandboxAccountStatusResult>{inner_xml}</GetSMSSandboxAccountStatusResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetSMSSandboxAccountStatusResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetSMSSandboxAccountStatusResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_subscription_attributes_response(
    result: &GetSubscriptionAttributesResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<GetSubscriptionAttributesResult>{inner_xml}</GetSubscriptionAttributesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetSubscriptionAttributesResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetSubscriptionAttributesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_topic_attributes_response(
    result: &GetTopicAttributesResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetTopicAttributesResult>{inner_xml}</GetTopicAttributesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetTopicAttributesResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetTopicAttributesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_endpoints_by_platform_application_response(
    result: &ListEndpointsByPlatformApplicationResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ListEndpointsByPlatformApplicationResult>{inner_xml}</ListEndpointsByPlatformApplicationResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListEndpointsByPlatformApplicationResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListEndpointsByPlatformApplicationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_origination_numbers_response(
    result: &ListOriginationNumbersResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListOriginationNumbersResult>{inner_xml}</ListOriginationNumbersResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListOriginationNumbersResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListOriginationNumbersResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_phone_numbers_opted_out_response(
    result: &ListPhoneNumbersOptedOutResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListPhoneNumbersOptedOutResult>{inner_xml}</ListPhoneNumbersOptedOutResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListPhoneNumbersOptedOutResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListPhoneNumbersOptedOutResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_platform_applications_response(
    result: &ListPlatformApplicationsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListPlatformApplicationsResult>{inner_xml}</ListPlatformApplicationsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListPlatformApplicationsResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListPlatformApplicationsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_s_m_s_sandbox_phone_numbers_response(
    result: &ListSMSSandboxPhoneNumbersResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListSMSSandboxPhoneNumbersResult>{inner_xml}</ListSMSSandboxPhoneNumbersResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListSMSSandboxPhoneNumbersResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListSMSSandboxPhoneNumbersResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_subscriptions_response(result: &ListSubscriptionsResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListSubscriptionsResult>{inner_xml}</ListSubscriptionsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListSubscriptionsResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListSubscriptionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_subscriptions_by_topic_response(
    result: &ListSubscriptionsByTopicResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListSubscriptionsByTopicResult>{inner_xml}</ListSubscriptionsByTopicResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListSubscriptionsByTopicResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListSubscriptionsByTopicResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListTagsForResourceResult>{inner_xml}</ListTagsForResourceResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListTagsForResourceResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListTagsForResourceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_topics_response(result: &ListTopicsResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListTopicsResult>{inner_xml}</ListTopicsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListTopicsResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListTopicsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_opt_in_phone_number_response(result: &OptInPhoneNumberResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<OptInPhoneNumberResult>{inner_xml}</OptInPhoneNumberResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<OptInPhoneNumberResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</OptInPhoneNumberResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_publish_response(result: &PublishResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<PublishResult>{inner_xml}</PublishResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PublishResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PublishResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_publish_batch_response(result: &PublishBatchResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<PublishBatchResult>{inner_xml}</PublishBatchResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PublishBatchResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PublishBatchResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_put_data_protection_policy_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PutDataProtectionPolicyResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PutDataProtectionPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_remove_permission_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RemovePermissionResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RemovePermissionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_set_endpoint_attributes_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetEndpointAttributesResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetEndpointAttributesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_set_platform_application_attributes_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetPlatformApplicationAttributesResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetPlatformApplicationAttributesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_set_s_m_s_attributes_response(result: &SetSMSAttributesResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<SetSMSAttributesResult>{inner_xml}</SetSMSAttributesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetSMSAttributesResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetSMSAttributesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_set_subscription_attributes_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetSubscriptionAttributesResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetSubscriptionAttributesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_set_topic_attributes_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetTopicAttributesResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetTopicAttributesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_subscribe_response(result: &SubscribeResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<SubscribeResult>{inner_xml}</SubscribeResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SubscribeResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SubscribeResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<TagResourceResult>{inner_xml}</TagResourceResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<TagResourceResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</TagResourceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_unsubscribe_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UnsubscribeResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UnsubscribeResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<UntagResourceResult>{inner_xml}</UntagResourceResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UntagResourceResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UntagResourceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_verify_s_m_s_sandbox_phone_number_response(
    result: &VerifySMSSandboxPhoneNumberResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<VerifySMSSandboxPhoneNumberResult>{inner_xml}</VerifySMSSandboxPhoneNumberResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<VerifySMSSandboxPhoneNumberResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</VerifySMSSandboxPhoneNumberResponse>"#
    );
    MockResponse::xml(200, xml)
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

fn deserialize_publish_batch_request_entry_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<PublishBatchRequestEntry>, String> {
    let mut item = PublishBatchRequestEntry::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Id")) {
        item.id = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Message")) {
        item.message = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.MessageDeduplicationId")) {
        item.message_deduplication_id = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.MessageGroupId")) {
        item.message_group_id = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.MessageStructure")) {
        item.message_structure = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Subject")) {
        item.subject = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

/// Deserialize awsQuery request for AddPermission.
pub fn deserialize_add_permission_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AddPermissionInput, String> {
    let mut input = AddPermissionInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "AWSAccountId".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.a_w_s_account_id = DelegatesList { items };
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ActionName".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.action_name = ActionsList { items };
        }
    }
    if let Some(value) = params.get("Label") {
        input.label = value.to_string();
    }
    if let Some(value) = params.get("TopicArn") {
        input.topic_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CheckIfPhoneNumberIsOptedOut.
pub fn deserialize_check_if_phone_number_is_opted_out_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CheckIfPhoneNumberIsOptedOutInput, String> {
    let mut input = CheckIfPhoneNumberIsOptedOutInput::default();
    if let Some(value) = params.get("phoneNumber") {
        input.phone_number = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ConfirmSubscription.
pub fn deserialize_confirm_subscription_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ConfirmSubscriptionInput, String> {
    let mut input = ConfirmSubscriptionInput::default();
    if let Some(value) = params.get("AuthenticateOnUnsubscribe") {
        input.authenticate_on_unsubscribe = Some(value.to_string());
    }
    if let Some(value) = params.get("Token") {
        input.token = value.to_string();
    }
    if let Some(value) = params.get("TopicArn") {
        input.topic_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreatePlatformApplication.
pub fn deserialize_create_platform_application_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreatePlatformApplicationInput, String> {
    let mut input = CreatePlatformApplicationInput::default();
    if let Some(value) = params.get("Name") {
        input.name = value.to_string();
    }
    if let Some(value) = params.get("Platform") {
        input.platform = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreatePlatformEndpoint.
pub fn deserialize_create_platform_endpoint_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreatePlatformEndpointInput, String> {
    let mut input = CreatePlatformEndpointInput::default();
    if let Some(value) = params.get("CustomUserData") {
        input.custom_user_data = Some(value.to_string());
    }
    if let Some(value) = params.get("PlatformApplicationArn") {
        input.platform_application_arn = value.to_string();
    }
    if let Some(value) = params.get("Token") {
        input.token = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateSMSSandboxPhoneNumber.
pub fn deserialize_create_s_m_s_sandbox_phone_number_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateSMSSandboxPhoneNumberInput, String> {
    let mut input = CreateSMSSandboxPhoneNumberInput::default();
    if let Some(value) = params.get("LanguageCode") {
        input.language_code = Some(value.to_string());
    }
    if let Some(value) = params.get("PhoneNumber") {
        input.phone_number = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateTopic.
pub fn deserialize_create_topic_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateTopicInput, String> {
    let mut input = CreateTopicInput::default();
    if let Some(value) = params.get("DataProtectionPolicy") {
        input.data_protection_policy = Some(value.to_string());
    }
    if let Some(value) = params.get("Name") {
        input.name = value.to_string();
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
            input.tags = Some(TagList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteEndpoint.
pub fn deserialize_delete_endpoint_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteEndpointInput, String> {
    let mut input = DeleteEndpointInput::default();
    if let Some(value) = params.get("EndpointArn") {
        input.endpoint_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeletePlatformApplication.
pub fn deserialize_delete_platform_application_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeletePlatformApplicationInput, String> {
    let mut input = DeletePlatformApplicationInput::default();
    if let Some(value) = params.get("PlatformApplicationArn") {
        input.platform_application_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteSMSSandboxPhoneNumber.
pub fn deserialize_delete_s_m_s_sandbox_phone_number_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteSMSSandboxPhoneNumberInput, String> {
    let mut input = DeleteSMSSandboxPhoneNumberInput::default();
    if let Some(value) = params.get("PhoneNumber") {
        input.phone_number = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteTopic.
pub fn deserialize_delete_topic_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteTopicInput, String> {
    let mut input = DeleteTopicInput::default();
    if let Some(value) = params.get("TopicArn") {
        input.topic_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetDataProtectionPolicy.
pub fn deserialize_get_data_protection_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetDataProtectionPolicyInput, String> {
    let mut input = GetDataProtectionPolicyInput::default();
    if let Some(value) = params.get("ResourceArn") {
        input.resource_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetEndpointAttributes.
pub fn deserialize_get_endpoint_attributes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetEndpointAttributesInput, String> {
    let mut input = GetEndpointAttributesInput::default();
    if let Some(value) = params.get("EndpointArn") {
        input.endpoint_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetPlatformApplicationAttributes.
pub fn deserialize_get_platform_application_attributes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetPlatformApplicationAttributesInput, String> {
    let mut input = GetPlatformApplicationAttributesInput::default();
    if let Some(value) = params.get("PlatformApplicationArn") {
        input.platform_application_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetSMSAttributes.
pub fn deserialize_get_s_m_s_attributes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetSMSAttributesInput, String> {
    let mut input = GetSMSAttributesInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "attributes".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.attributes = Some(ListString { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetSMSSandboxAccountStatus.
pub fn deserialize_get_s_m_s_sandbox_account_status_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetSMSSandboxAccountStatusInput, String> {
    let input = GetSMSSandboxAccountStatusInput {};
    Ok(input)
}

/// Deserialize awsQuery request for GetSubscriptionAttributes.
pub fn deserialize_get_subscription_attributes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetSubscriptionAttributesInput, String> {
    let mut input = GetSubscriptionAttributesInput::default();
    if let Some(value) = params.get("SubscriptionArn") {
        input.subscription_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetTopicAttributes.
pub fn deserialize_get_topic_attributes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetTopicAttributesInput, String> {
    let mut input = GetTopicAttributesInput::default();
    if let Some(value) = params.get("TopicArn") {
        input.topic_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListEndpointsByPlatformApplication.
pub fn deserialize_list_endpoints_by_platform_application_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListEndpointsByPlatformApplicationInput, String> {
    let mut input = ListEndpointsByPlatformApplicationInput::default();
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("PlatformApplicationArn") {
        input.platform_application_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListOriginationNumbers.
pub fn deserialize_list_origination_numbers_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListOriginationNumbersRequest, String> {
    let mut input = ListOriginationNumbersRequest::default();
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

/// Deserialize awsQuery request for ListPhoneNumbersOptedOut.
pub fn deserialize_list_phone_numbers_opted_out_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListPhoneNumbersOptedOutInput, String> {
    let mut input = ListPhoneNumbersOptedOutInput::default();
    if let Some(value) = params.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListPlatformApplications.
pub fn deserialize_list_platform_applications_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListPlatformApplicationsInput, String> {
    let mut input = ListPlatformApplicationsInput::default();
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListSMSSandboxPhoneNumbers.
pub fn deserialize_list_s_m_s_sandbox_phone_numbers_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListSMSSandboxPhoneNumbersInput, String> {
    let mut input = ListSMSSandboxPhoneNumbersInput::default();
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

/// Deserialize awsQuery request for ListSubscriptions.
pub fn deserialize_list_subscriptions_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListSubscriptionsInput, String> {
    let mut input = ListSubscriptionsInput::default();
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListSubscriptionsByTopic.
pub fn deserialize_list_subscriptions_by_topic_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListSubscriptionsByTopicInput, String> {
    let mut input = ListSubscriptionsByTopicInput::default();
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("TopicArn") {
        input.topic_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListTagsForResource.
pub fn deserialize_list_tags_for_resource_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListTagsForResourceRequest, String> {
    let mut input = ListTagsForResourceRequest::default();
    if let Some(value) = params.get("ResourceArn") {
        input.resource_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListTopics.
pub fn deserialize_list_topics_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListTopicsInput, String> {
    let mut input = ListTopicsInput::default();
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for OptInPhoneNumber.
pub fn deserialize_opt_in_phone_number_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<OptInPhoneNumberInput, String> {
    let mut input = OptInPhoneNumberInput::default();
    if let Some(value) = params.get("phoneNumber") {
        input.phone_number = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for Publish.
pub fn deserialize_publish_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<PublishInput, String> {
    let mut input = PublishInput::default();
    if let Some(value) = params.get("Message") {
        input.message = value.to_string();
    }
    if let Some(value) = params.get("MessageDeduplicationId") {
        input.message_deduplication_id = Some(value.to_string());
    }
    if let Some(value) = params.get("MessageGroupId") {
        input.message_group_id = Some(value.to_string());
    }
    if let Some(value) = params.get("MessageStructure") {
        input.message_structure = Some(value.to_string());
    }
    if let Some(value) = params.get("PhoneNumber") {
        input.phone_number = Some(value.to_string());
    }
    if let Some(value) = params.get("Subject") {
        input.subject = Some(value.to_string());
    }
    if let Some(value) = params.get("TargetArn") {
        input.target_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("TopicArn") {
        input.topic_arn = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for PublishBatch.
pub fn deserialize_publish_batch_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<PublishBatchInput, String> {
    let mut input = PublishBatchInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "PublishBatchRequestEntries".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_publish_batch_request_entry_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.publish_batch_request_entries = PublishBatchRequestEntryList { items };
        }
    }
    if let Some(value) = params.get("TopicArn") {
        input.topic_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for PutDataProtectionPolicy.
pub fn deserialize_put_data_protection_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<PutDataProtectionPolicyInput, String> {
    let mut input = PutDataProtectionPolicyInput::default();
    if let Some(value) = params.get("DataProtectionPolicy") {
        input.data_protection_policy = value.to_string();
    }
    if let Some(value) = params.get("ResourceArn") {
        input.resource_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for RemovePermission.
pub fn deserialize_remove_permission_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RemovePermissionInput, String> {
    let mut input = RemovePermissionInput::default();
    if let Some(value) = params.get("Label") {
        input.label = value.to_string();
    }
    if let Some(value) = params.get("TopicArn") {
        input.topic_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for SetEndpointAttributes.
pub fn deserialize_set_endpoint_attributes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetEndpointAttributesInput, String> {
    let mut input = SetEndpointAttributesInput::default();
    if let Some(value) = params.get("EndpointArn") {
        input.endpoint_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for SetPlatformApplicationAttributes.
pub fn deserialize_set_platform_application_attributes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetPlatformApplicationAttributesInput, String> {
    let mut input = SetPlatformApplicationAttributesInput::default();
    if let Some(value) = params.get("PlatformApplicationArn") {
        input.platform_application_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for SetSMSAttributes.
pub fn deserialize_set_s_m_s_attributes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetSMSAttributesInput, String> {
    let mut input = SetSMSAttributesInput::default();
    Ok(input)
}

/// Deserialize awsQuery request for SetSubscriptionAttributes.
pub fn deserialize_set_subscription_attributes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetSubscriptionAttributesInput, String> {
    let mut input = SetSubscriptionAttributesInput::default();
    if let Some(value) = params.get("AttributeName") {
        input.attribute_name = value.to_string();
    }
    if let Some(value) = params.get("AttributeValue") {
        input.attribute_value = Some(value.to_string());
    }
    if let Some(value) = params.get("SubscriptionArn") {
        input.subscription_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for SetTopicAttributes.
pub fn deserialize_set_topic_attributes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetTopicAttributesInput, String> {
    let mut input = SetTopicAttributesInput::default();
    if let Some(value) = params.get("AttributeName") {
        input.attribute_name = value.to_string();
    }
    if let Some(value) = params.get("AttributeValue") {
        input.attribute_value = Some(value.to_string());
    }
    if let Some(value) = params.get("TopicArn") {
        input.topic_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for Subscribe.
pub fn deserialize_subscribe_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SubscribeInput, String> {
    let mut input = SubscribeInput::default();
    if let Some(value) = params.get("Endpoint") {
        input.endpoint = Some(value.to_string());
    }
    if let Some(value) = params.get("Protocol") {
        input.protocol = value.to_string();
    }
    if let Some(value) = params.get("ReturnSubscriptionArn") {
        input.return_subscription_arn = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ReturnSubscriptionArn: {e}"))?,
        );
    }
    if let Some(value) = params.get("TopicArn") {
        input.topic_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for TagResource.
pub fn deserialize_tag_resource_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<TagResourceRequest, String> {
    let mut input = TagResourceRequest::default();
    if let Some(value) = params.get("ResourceArn") {
        input.resource_arn = value.to_string();
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
            input.tags = TagList { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for Unsubscribe.
pub fn deserialize_unsubscribe_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UnsubscribeInput, String> {
    let mut input = UnsubscribeInput::default();
    if let Some(value) = params.get("SubscriptionArn") {
        input.subscription_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for UntagResource.
pub fn deserialize_untag_resource_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UntagResourceRequest, String> {
    let mut input = UntagResourceRequest::default();
    if let Some(value) = params.get("ResourceArn") {
        input.resource_arn = value.to_string();
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
            input.tag_keys = TagKeyList { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for VerifySMSSandboxPhoneNumber.
pub fn deserialize_verify_s_m_s_sandbox_phone_number_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<VerifySMSSandboxPhoneNumberInput, String> {
    let mut input = VerifySMSSandboxPhoneNumberInput::default();
    if let Some(value) = params.get("OneTimePassword") {
        input.one_time_password = value.to_string();
    }
    if let Some(value) = params.get("PhoneNumber") {
        input.phone_number = value.to_string();
    }
    Ok(input)
}
