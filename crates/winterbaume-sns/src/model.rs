//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-sns

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AddPermissionInput")]
pub struct AddPermissionInput {
    #[serde(rename = "AWSAccountId")]
    #[serde(default)]
    pub a_w_s_account_id: DelegatesList,
    #[serde(rename = "ActionName")]
    #[serde(default)]
    pub action_name: ActionsList,
    #[serde(rename = "Label")]
    #[serde(default)]
    pub label: String,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    pub topic_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DelegatesList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for DelegatesList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for DelegatesList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<String>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlStringList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<String>,
}

impl From<Vec<String>> for XmlStringList {
    fn from(v: Vec<String>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<String> for XmlStringList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionsList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ActionsList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ActionsList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CheckIfPhoneNumberIsOptedOutInput")]
pub struct CheckIfPhoneNumberIsOptedOutInput {
    #[serde(rename = "phoneNumber")]
    #[serde(default)]
    pub phone_number: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CheckIfPhoneNumberIsOptedOutResult")]
pub struct CheckIfPhoneNumberIsOptedOutResponse {
    #[serde(rename = "isOptedOut")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_opted_out: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ConfirmSubscriptionInput")]
pub struct ConfirmSubscriptionInput {
    #[serde(rename = "AuthenticateOnUnsubscribe")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticate_on_unsubscribe: Option<String>,
    #[serde(rename = "Token")]
    #[serde(default)]
    pub token: String,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    pub topic_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ConfirmSubscriptionResult")]
pub struct ConfirmSubscriptionResponse {
    #[serde(rename = "SubscriptionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreatePlatformEndpointResult")]
pub struct CreateEndpointResponse {
    #[serde(rename = "EndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreatePlatformApplicationInput")]
pub struct CreatePlatformApplicationInput {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    pub attributes: std::collections::HashMap<String, String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Platform")]
    #[serde(default)]
    pub platform: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreatePlatformApplicationResult")]
pub struct CreatePlatformApplicationResponse {
    #[serde(rename = "PlatformApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_application_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreatePlatformEndpointInput")]
pub struct CreatePlatformEndpointInput {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "CustomUserData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_user_data: Option<String>,
    #[serde(rename = "PlatformApplicationArn")]
    #[serde(default)]
    pub platform_application_arn: String,
    #[serde(rename = "Token")]
    #[serde(default)]
    pub token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateSMSSandboxPhoneNumberInput")]
pub struct CreateSMSSandboxPhoneNumberInput {
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "PhoneNumber")]
    #[serde(default)]
    pub phone_number: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSMSSandboxPhoneNumberResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateTopicInput")]
pub struct CreateTopicInput {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "DataProtectionPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_protection_policy: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Tag>,
}
impl From<Vec<Tag>> for TagList {
    fn from(v: Vec<Tag>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Tag> for TagList {
    fn from_iter<I: IntoIterator<Item = Tag>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Tag>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTagList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Tag>,
}

impl From<Vec<Tag>> for XmlTagList {
    fn from(v: Vec<Tag>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Tag> for XmlTagList {
    fn from_iter<I: IntoIterator<Item = Tag>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Tag")]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateTopicResult")]
pub struct CreateTopicResponse {
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteEndpointInput")]
pub struct DeleteEndpointInput {
    #[serde(rename = "EndpointArn")]
    #[serde(default)]
    pub endpoint_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeletePlatformApplicationInput")]
pub struct DeletePlatformApplicationInput {
    #[serde(rename = "PlatformApplicationArn")]
    #[serde(default)]
    pub platform_application_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteSMSSandboxPhoneNumberInput")]
pub struct DeleteSMSSandboxPhoneNumberInput {
    #[serde(rename = "PhoneNumber")]
    #[serde(default)]
    pub phone_number: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSMSSandboxPhoneNumberResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteTopicInput")]
pub struct DeleteTopicInput {
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    pub topic_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetDataProtectionPolicyInput")]
pub struct GetDataProtectionPolicyInput {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetDataProtectionPolicyResult")]
pub struct GetDataProtectionPolicyResponse {
    #[serde(rename = "DataProtectionPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_protection_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetEndpointAttributesInput")]
pub struct GetEndpointAttributesInput {
    #[serde(rename = "EndpointArn")]
    #[serde(default)]
    pub endpoint_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetEndpointAttributesResult")]
pub struct GetEndpointAttributesResponse {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetPlatformApplicationAttributesInput")]
pub struct GetPlatformApplicationAttributesInput {
    #[serde(rename = "PlatformApplicationArn")]
    #[serde(default)]
    pub platform_application_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetPlatformApplicationAttributesResult")]
pub struct GetPlatformApplicationAttributesResponse {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetSMSAttributesInput")]
pub struct GetSMSAttributesInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<ListString>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListString {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ListString {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ListString {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetSMSAttributesResult")]
pub struct GetSMSAttributesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSMSSandboxAccountStatusInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetSMSSandboxAccountStatusResult")]
pub struct GetSMSSandboxAccountStatusResult {
    #[serde(rename = "IsInSandbox")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_in_sandbox: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetSubscriptionAttributesInput")]
pub struct GetSubscriptionAttributesInput {
    #[serde(rename = "SubscriptionArn")]
    #[serde(default)]
    pub subscription_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetSubscriptionAttributesResult")]
pub struct GetSubscriptionAttributesResponse {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetTopicAttributesInput")]
pub struct GetTopicAttributesInput {
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    pub topic_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetTopicAttributesResult")]
pub struct GetTopicAttributesResponse {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListEndpointsByPlatformApplicationInput")]
pub struct ListEndpointsByPlatformApplicationInput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PlatformApplicationArn")]
    #[serde(default)]
    pub platform_application_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListEndpointsByPlatformApplicationResult")]
pub struct ListEndpointsByPlatformApplicationResponse {
    #[serde(rename = "Endpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<ListOfEndpoints>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOfEndpoints {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Endpoint>,
}
impl From<Vec<Endpoint>> for ListOfEndpoints {
    fn from(v: Vec<Endpoint>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Endpoint> for ListOfEndpoints {
    fn from_iter<I: IntoIterator<Item = Endpoint>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Endpoint>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlEndpointList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Endpoint>,
}

impl From<Vec<Endpoint>> for XmlEndpointList {
    fn from(v: Vec<Endpoint>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Endpoint> for XmlEndpointList {
    fn from_iter<I: IntoIterator<Item = Endpoint>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Endpoint")]
pub struct Endpoint {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "EndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListOriginationNumbersRequest")]
pub struct ListOriginationNumbersRequest {
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
#[serde(rename = "ListOriginationNumbersResult")]
pub struct ListOriginationNumbersResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PhoneNumbers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<PhoneNumberInformationList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PhoneNumberInformationList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PhoneNumberInformation>,
}
impl From<Vec<PhoneNumberInformation>> for PhoneNumberInformationList {
    fn from(v: Vec<PhoneNumberInformation>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PhoneNumberInformation> for PhoneNumberInformationList {
    fn from_iter<I: IntoIterator<Item = PhoneNumberInformation>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PhoneNumberInformation>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPhoneNumberInformationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PhoneNumberInformation>,
}

impl From<Vec<PhoneNumberInformation>> for XmlPhoneNumberInformationList {
    fn from(v: Vec<PhoneNumberInformation>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PhoneNumberInformation> for XmlPhoneNumberInformationList {
    fn from_iter<I: IntoIterator<Item = PhoneNumberInformation>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PhoneNumberInformation")]
pub struct PhoneNumberInformation {
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "Iso2CountryCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso2_country_code: Option<String>,
    #[serde(rename = "NumberCapabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_capabilities: Option<NumberCapabilityList>,
    #[serde(rename = "PhoneNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "RouteType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NumberCapabilityList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for NumberCapabilityList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for NumberCapabilityList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListPhoneNumbersOptedOutInput")]
pub struct ListPhoneNumbersOptedOutInput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListPhoneNumbersOptedOutResult")]
pub struct ListPhoneNumbersOptedOutResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "phoneNumbers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<PhoneNumberList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PhoneNumberList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for PhoneNumberList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for PhoneNumberList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListPlatformApplicationsInput")]
pub struct ListPlatformApplicationsInput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListPlatformApplicationsResult")]
pub struct ListPlatformApplicationsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PlatformApplications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_applications: Option<ListOfPlatformApplications>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOfPlatformApplications {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PlatformApplication>,
}
impl From<Vec<PlatformApplication>> for ListOfPlatformApplications {
    fn from(v: Vec<PlatformApplication>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PlatformApplication> for ListOfPlatformApplications {
    fn from_iter<I: IntoIterator<Item = PlatformApplication>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PlatformApplication>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPlatformApplicationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PlatformApplication>,
}

impl From<Vec<PlatformApplication>> for XmlPlatformApplicationList {
    fn from(v: Vec<PlatformApplication>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PlatformApplication> for XmlPlatformApplicationList {
    fn from_iter<I: IntoIterator<Item = PlatformApplication>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PlatformApplication")]
pub struct PlatformApplication {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "PlatformApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_application_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListSMSSandboxPhoneNumbersInput")]
pub struct ListSMSSandboxPhoneNumbersInput {
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
#[serde(rename = "ListSMSSandboxPhoneNumbersResult")]
pub struct ListSMSSandboxPhoneNumbersResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PhoneNumbers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<SMSSandboxPhoneNumberList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SMSSandboxPhoneNumberList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<SMSSandboxPhoneNumber>,
}
impl From<Vec<SMSSandboxPhoneNumber>> for SMSSandboxPhoneNumberList {
    fn from(v: Vec<SMSSandboxPhoneNumber>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<SMSSandboxPhoneNumber> for SMSSandboxPhoneNumberList {
    fn from_iter<I: IntoIterator<Item = SMSSandboxPhoneNumber>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<SMSSandboxPhoneNumber>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSMSSandboxPhoneNumberList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<SMSSandboxPhoneNumber>,
}

impl From<Vec<SMSSandboxPhoneNumber>> for XmlSMSSandboxPhoneNumberList {
    fn from(v: Vec<SMSSandboxPhoneNumber>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<SMSSandboxPhoneNumber> for XmlSMSSandboxPhoneNumberList {
    fn from_iter<I: IntoIterator<Item = SMSSandboxPhoneNumber>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SMSSandboxPhoneNumber")]
pub struct SMSSandboxPhoneNumber {
    #[serde(rename = "PhoneNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListSubscriptionsByTopicInput")]
pub struct ListSubscriptionsByTopicInput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    pub topic_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListSubscriptionsByTopicResult")]
pub struct ListSubscriptionsByTopicResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Subscriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<SubscriptionsList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubscriptionsList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Subscription>,
}
impl From<Vec<Subscription>> for SubscriptionsList {
    fn from(v: Vec<Subscription>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Subscription> for SubscriptionsList {
    fn from_iter<I: IntoIterator<Item = Subscription>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Subscription>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSubscriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Subscription>,
}

impl From<Vec<Subscription>> for XmlSubscriptionList {
    fn from(v: Vec<Subscription>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Subscription> for XmlSubscriptionList {
    fn from_iter<I: IntoIterator<Item = Subscription>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Subscription")]
pub struct Subscription {
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "SubscriptionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_arn: Option<String>,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListSubscriptionsInput")]
pub struct ListSubscriptionsInput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListSubscriptionsResult")]
pub struct ListSubscriptionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Subscriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<SubscriptionsList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTagsForResourceRequest")]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTagsForResourceResult")]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTopicsInput")]
pub struct ListTopicsInput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTopicsResult")]
pub struct ListTopicsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Topics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<TopicsList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicsList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Topic>,
}
impl From<Vec<Topic>> for TopicsList {
    fn from(v: Vec<Topic>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Topic> for TopicsList {
    fn from_iter<I: IntoIterator<Item = Topic>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Topic>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTopicList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Topic>,
}

impl From<Vec<Topic>> for XmlTopicList {
    fn from(v: Vec<Topic>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Topic> for XmlTopicList {
    fn from_iter<I: IntoIterator<Item = Topic>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Topic")]
pub struct Topic {
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OptInPhoneNumberInput")]
pub struct OptInPhoneNumberInput {
    #[serde(rename = "phoneNumber")]
    #[serde(default)]
    pub phone_number: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OptInPhoneNumberResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PublishBatchInput")]
pub struct PublishBatchInput {
    #[serde(rename = "PublishBatchRequestEntries")]
    #[serde(default)]
    pub publish_batch_request_entries: PublishBatchRequestEntryList,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    pub topic_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublishBatchRequestEntryList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PublishBatchRequestEntry>,
}
impl From<Vec<PublishBatchRequestEntry>> for PublishBatchRequestEntryList {
    fn from(v: Vec<PublishBatchRequestEntry>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PublishBatchRequestEntry> for PublishBatchRequestEntryList {
    fn from_iter<I: IntoIterator<Item = PublishBatchRequestEntry>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PublishBatchRequestEntry>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPublishBatchRequestEntryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PublishBatchRequestEntry>,
}

impl From<Vec<PublishBatchRequestEntry>> for XmlPublishBatchRequestEntryList {
    fn from(v: Vec<PublishBatchRequestEntry>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PublishBatchRequestEntry> for XmlPublishBatchRequestEntryList {
    fn from_iter<I: IntoIterator<Item = PublishBatchRequestEntry>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PublishBatchRequestEntry")]
pub struct PublishBatchRequestEntry {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Message")]
    #[serde(default)]
    pub message: String,
    #[serde(rename = "MessageAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_attributes: Option<std::collections::HashMap<String, MessageAttributeValue>>,
    #[serde(rename = "MessageDeduplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_deduplication_id: Option<String>,
    #[serde(rename = "MessageGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_group_id: Option<String>,
    #[serde(rename = "MessageStructure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_structure: Option<String>,
    #[serde(rename = "Subject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MessageAttributeValue")]
pub struct MessageAttributeValue {
    #[serde(rename = "BinaryValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_value: Option<String>,
    #[serde(rename = "DataType")]
    #[serde(default)]
    pub data_type: String,
    #[serde(rename = "StringValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PublishBatchResult")]
pub struct PublishBatchResponse {
    #[serde(rename = "Failed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<BatchResultErrorEntryList>,
    #[serde(rename = "Successful")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<PublishBatchResultEntryList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchResultErrorEntryList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<BatchResultErrorEntry>,
}
impl From<Vec<BatchResultErrorEntry>> for BatchResultErrorEntryList {
    fn from(v: Vec<BatchResultErrorEntry>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<BatchResultErrorEntry> for BatchResultErrorEntryList {
    fn from_iter<I: IntoIterator<Item = BatchResultErrorEntry>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<BatchResultErrorEntry>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlBatchResultErrorEntryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<BatchResultErrorEntry>,
}

impl From<Vec<BatchResultErrorEntry>> for XmlBatchResultErrorEntryList {
    fn from(v: Vec<BatchResultErrorEntry>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<BatchResultErrorEntry> for XmlBatchResultErrorEntryList {
    fn from_iter<I: IntoIterator<Item = BatchResultErrorEntry>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BatchResultErrorEntry")]
pub struct BatchResultErrorEntry {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "SenderFault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_fault: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublishBatchResultEntryList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PublishBatchResultEntry>,
}
impl From<Vec<PublishBatchResultEntry>> for PublishBatchResultEntryList {
    fn from(v: Vec<PublishBatchResultEntry>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PublishBatchResultEntry> for PublishBatchResultEntryList {
    fn from_iter<I: IntoIterator<Item = PublishBatchResultEntry>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PublishBatchResultEntry>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPublishBatchResultEntryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PublishBatchResultEntry>,
}

impl From<Vec<PublishBatchResultEntry>> for XmlPublishBatchResultEntryList {
    fn from(v: Vec<PublishBatchResultEntry>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PublishBatchResultEntry> for XmlPublishBatchResultEntryList {
    fn from_iter<I: IntoIterator<Item = PublishBatchResultEntry>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PublishBatchResultEntry")]
pub struct PublishBatchResultEntry {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "MessageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename = "SequenceNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PublishInput")]
pub struct PublishInput {
    #[serde(rename = "Message")]
    #[serde(default)]
    pub message: String,
    #[serde(rename = "MessageAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_attributes: Option<std::collections::HashMap<String, MessageAttributeValue>>,
    #[serde(rename = "MessageDeduplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_deduplication_id: Option<String>,
    #[serde(rename = "MessageGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_group_id: Option<String>,
    #[serde(rename = "MessageStructure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_structure: Option<String>,
    #[serde(rename = "PhoneNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "Subject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PublishResult")]
pub struct PublishResponse {
    #[serde(rename = "MessageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename = "SequenceNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutDataProtectionPolicyInput")]
pub struct PutDataProtectionPolicyInput {
    #[serde(rename = "DataProtectionPolicy")]
    #[serde(default)]
    pub data_protection_policy: String,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RemovePermissionInput")]
pub struct RemovePermissionInput {
    #[serde(rename = "Label")]
    #[serde(default)]
    pub label: String,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    pub topic_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetEndpointAttributesInput")]
pub struct SetEndpointAttributesInput {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    pub attributes: std::collections::HashMap<String, String>,
    #[serde(rename = "EndpointArn")]
    #[serde(default)]
    pub endpoint_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetPlatformApplicationAttributesInput")]
pub struct SetPlatformApplicationAttributesInput {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    pub attributes: std::collections::HashMap<String, String>,
    #[serde(rename = "PlatformApplicationArn")]
    #[serde(default)]
    pub platform_application_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetSMSAttributesInput")]
pub struct SetSMSAttributesInput {
    #[serde(default)]
    pub attributes: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetSMSAttributesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetSubscriptionAttributesInput")]
pub struct SetSubscriptionAttributesInput {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "AttributeValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_value: Option<String>,
    #[serde(rename = "SubscriptionArn")]
    #[serde(default)]
    pub subscription_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetTopicAttributesInput")]
pub struct SetTopicAttributesInput {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "AttributeValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_value: Option<String>,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    pub topic_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SubscribeInput")]
pub struct SubscribeInput {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    pub protocol: String,
    #[serde(rename = "ReturnSubscriptionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_subscription_arn: Option<bool>,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    pub topic_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SubscribeResult")]
pub struct SubscribeResponse {
    #[serde(rename = "SubscriptionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TagResourceRequest")]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: TagList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UnsubscribeInput")]
pub struct UnsubscribeInput {
    #[serde(rename = "SubscriptionArn")]
    #[serde(default)]
    pub subscription_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UntagResourceRequest")]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: TagKeyList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagKeyList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for TagKeyList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for TagKeyList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "VerifySMSSandboxPhoneNumberInput")]
pub struct VerifySMSSandboxPhoneNumberInput {
    #[serde(rename = "OneTimePassword")]
    #[serde(default)]
    pub one_time_password: String,
    #[serde(rename = "PhoneNumber")]
    #[serde(default)]
    pub phone_number: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VerifySMSSandboxPhoneNumberResult {}
