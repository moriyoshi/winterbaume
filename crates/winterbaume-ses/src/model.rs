//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-ses

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetIdentityFeedbackForwardingEnabledRequest")]
pub struct SetIdentityFeedbackForwardingEnabledRequest {
    #[serde(rename = "ForwardingEnabled")]
    #[serde(default)]
    pub forwarding_enabled: bool,
    #[serde(rename = "Identity")]
    #[serde(default)]
    pub identity: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteIdentityRequest")]
pub struct DeleteIdentityRequest {
    #[serde(rename = "Identity")]
    #[serde(default)]
    pub identity: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListIdentitiesResult")]
pub struct ListIdentitiesResponse {
    #[serde(rename = "Identities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identities: Option<IdentityList>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IdentityList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for IdentityList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for IdentityList {
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
#[serde(rename = "DeleteIdentityPolicyRequest")]
pub struct DeleteIdentityPolicyRequest {
    #[serde(rename = "Identity")]
    #[serde(default)]
    pub identity: String,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetIdentityVerificationAttributesResult")]
pub struct GetIdentityVerificationAttributesResponse {
    #[serde(rename = "VerificationAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_attributes:
        Option<std::collections::HashMap<String, IdentityVerificationAttributes>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "IdentityVerificationAttributes")]
pub struct IdentityVerificationAttributes {
    #[serde(rename = "VerificationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<String>,
    #[serde(rename = "VerificationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTemplateResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateCustomVerificationEmailTemplateRequest")]
pub struct CreateCustomVerificationEmailTemplateRequest {
    #[serde(rename = "FailureRedirectionURL")]
    #[serde(default)]
    pub failure_redirection_u_r_l: String,
    #[serde(rename = "FromEmailAddress")]
    #[serde(default)]
    pub from_email_address: String,
    #[serde(rename = "SuccessRedirectionURL")]
    #[serde(default)]
    pub success_redirection_u_r_l: String,
    #[serde(rename = "TemplateContent")]
    #[serde(default)]
    pub template_content: String,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
    #[serde(rename = "TemplateSubject")]
    #[serde(default)]
    pub template_subject: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetIdentityNotificationAttributesResult")]
pub struct GetIdentityNotificationAttributesResponse {
    #[serde(rename = "NotificationAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_attributes:
        Option<std::collections::HashMap<String, IdentityNotificationAttributes>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "IdentityNotificationAttributes")]
pub struct IdentityNotificationAttributes {
    #[serde(rename = "BounceTopic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounce_topic: Option<String>,
    #[serde(rename = "ComplaintTopic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complaint_topic: Option<String>,
    #[serde(rename = "DeliveryTopic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_topic: Option<String>,
    #[serde(rename = "ForwardingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwarding_enabled: Option<bool>,
    #[serde(rename = "HeadersInBounceNotificationsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers_in_bounce_notifications_enabled: Option<bool>,
    #[serde(rename = "HeadersInComplaintNotificationsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers_in_complaint_notifications_enabled: Option<bool>,
    #[serde(rename = "HeadersInDeliveryNotificationsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers_in_delivery_notifications_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIdentityResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListReceiptRuleSetsResult")]
pub struct ListReceiptRuleSetsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RuleSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_sets: Option<ReceiptRuleSetsLists>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReceiptRuleSetsLists {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ReceiptRuleSetMetadata>,
}
impl From<Vec<ReceiptRuleSetMetadata>> for ReceiptRuleSetsLists {
    fn from(v: Vec<ReceiptRuleSetMetadata>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ReceiptRuleSetMetadata> for ReceiptRuleSetsLists {
    fn from_iter<I: IntoIterator<Item = ReceiptRuleSetMetadata>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ReceiptRuleSetMetadata>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlReceiptRuleSetMetadataList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ReceiptRuleSetMetadata>,
}

impl From<Vec<ReceiptRuleSetMetadata>> for XmlReceiptRuleSetMetadataList {
    fn from(v: Vec<ReceiptRuleSetMetadata>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ReceiptRuleSetMetadata> for XmlReceiptRuleSetMetadataList {
    fn from_iter<I: IntoIterator<Item = ReceiptRuleSetMetadata>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReceiptRuleSetMetadata")]
pub struct ReceiptRuleSetMetadata {
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SendEmailResult")]
pub struct SendEmailResponse {
    #[serde(rename = "MessageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateAccountSendingEnabledRequest")]
pub struct UpdateAccountSendingEnabledRequest {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "VerifyEmailAddressRequest")]
pub struct VerifyEmailAddressRequest {
    #[serde(rename = "EmailAddress")]
    #[serde(default)]
    pub email_address: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetIdentityMailFromDomainResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetIdentityMailFromDomainAttributesResult")]
pub struct GetIdentityMailFromDomainAttributesResponse {
    #[serde(rename = "MailFromDomainAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_from_domain_attributes:
        Option<std::collections::HashMap<String, IdentityMailFromDomainAttributes>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "IdentityMailFromDomainAttributes")]
pub struct IdentityMailFromDomainAttributes {
    #[serde(rename = "BehaviorOnMXFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior_on_m_x_failure: Option<String>,
    #[serde(rename = "MailFromDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_from_domain: Option<String>,
    #[serde(rename = "MailFromDomainStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_from_domain_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConfigurationSetTrackingOptionsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetIdentityPoliciesResult")]
pub struct GetIdentityPoliciesResponse {
    #[serde(rename = "Policies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetIdentityNotificationAttributesRequest")]
pub struct GetIdentityNotificationAttributesRequest {
    #[serde(rename = "Identities")]
    #[serde(default)]
    pub identities: IdentityList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReorderReceiptRuleSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListConfigurationSetsResult")]
pub struct ListConfigurationSetsResponse {
    #[serde(rename = "ConfigurationSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_sets: Option<ConfigurationSets>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationSets {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ConfigurationSet>,
}
impl From<Vec<ConfigurationSet>> for ConfigurationSets {
    fn from(v: Vec<ConfigurationSet>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ConfigurationSet> for ConfigurationSets {
    fn from_iter<I: IntoIterator<Item = ConfigurationSet>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ConfigurationSet>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlConfigurationSetList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ConfigurationSet>,
}

impl From<Vec<ConfigurationSet>> for XmlConfigurationSetList {
    fn from(v: Vec<ConfigurationSet>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ConfigurationSet> for XmlConfigurationSetList {
    fn from_iter<I: IntoIterator<Item = ConfigurationSet>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ConfigurationSet")]
pub struct ConfigurationSet {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetIdentityMailFromDomainAttributesRequest")]
pub struct GetIdentityMailFromDomainAttributesRequest {
    #[serde(rename = "Identities")]
    #[serde(default)]
    pub identities: IdentityList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateConfigurationSetEventDestinationRequest")]
pub struct CreateConfigurationSetEventDestinationRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    pub configuration_set_name: String,
    #[serde(rename = "EventDestination")]
    #[serde(default)]
    pub event_destination: EventDestination,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EventDestination")]
pub struct EventDestination {
    #[serde(rename = "CloudWatchDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_destination: Option<CloudWatchDestination>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "KinesisFirehoseDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_destination: Option<KinesisFirehoseDestination>,
    #[serde(rename = "MatchingEventTypes")]
    #[serde(default)]
    pub matching_event_types: EventTypes,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "SNSDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_n_s_destination: Option<SNSDestination>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CloudWatchDestination")]
pub struct CloudWatchDestination {
    #[serde(rename = "DimensionConfigurations")]
    #[serde(default)]
    pub dimension_configurations: CloudWatchDimensionConfigurations,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchDimensionConfigurations {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CloudWatchDimensionConfiguration>,
}
impl From<Vec<CloudWatchDimensionConfiguration>> for CloudWatchDimensionConfigurations {
    fn from(v: Vec<CloudWatchDimensionConfiguration>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<CloudWatchDimensionConfiguration> for CloudWatchDimensionConfigurations {
    fn from_iter<I: IntoIterator<Item = CloudWatchDimensionConfiguration>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<CloudWatchDimensionConfiguration>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlCloudWatchDimensionConfigurationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<CloudWatchDimensionConfiguration>,
}

impl From<Vec<CloudWatchDimensionConfiguration>> for XmlCloudWatchDimensionConfigurationList {
    fn from(v: Vec<CloudWatchDimensionConfiguration>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<CloudWatchDimensionConfiguration> for XmlCloudWatchDimensionConfigurationList {
    fn from_iter<I: IntoIterator<Item = CloudWatchDimensionConfiguration>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CloudWatchDimensionConfiguration")]
pub struct CloudWatchDimensionConfiguration {
    #[serde(rename = "DefaultDimensionValue")]
    #[serde(default)]
    pub default_dimension_value: String,
    #[serde(rename = "DimensionName")]
    #[serde(default)]
    pub dimension_name: String,
    #[serde(rename = "DimensionValueSource")]
    #[serde(default)]
    pub dimension_value_source: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "KinesisFirehoseDestination")]
pub struct KinesisFirehoseDestination {
    #[serde(rename = "DeliveryStreamARN")]
    #[serde(default)]
    pub delivery_stream_a_r_n: String,
    #[serde(rename = "IAMRoleARN")]
    #[serde(default)]
    pub i_a_m_role_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventTypes {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for EventTypes {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for EventTypes {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SNSDestination")]
pub struct SNSDestination {
    #[serde(rename = "TopicARN")]
    #[serde(default)]
    pub topic_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetIdentityFeedbackForwardingEnabledResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeReceiptRuleSetResult")]
pub struct DescribeReceiptRuleSetResponse {
    #[serde(rename = "Metadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ReceiptRuleSetMetadata>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<ReceiptRulesList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReceiptRulesList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ReceiptRule>,
}
impl From<Vec<ReceiptRule>> for ReceiptRulesList {
    fn from(v: Vec<ReceiptRule>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ReceiptRule> for ReceiptRulesList {
    fn from_iter<I: IntoIterator<Item = ReceiptRule>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ReceiptRule>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlReceiptRuleList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ReceiptRule>,
}

impl From<Vec<ReceiptRule>> for XmlReceiptRuleList {
    fn from(v: Vec<ReceiptRule>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ReceiptRule> for XmlReceiptRuleList {
    fn from_iter<I: IntoIterator<Item = ReceiptRule>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReceiptRule")]
pub struct ReceiptRule {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<ReceiptActionsList>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Recipients")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<RecipientsList>,
    #[serde(rename = "ScanEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_enabled: Option<bool>,
    #[serde(rename = "TlsPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReceiptActionsList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ReceiptAction>,
}
impl From<Vec<ReceiptAction>> for ReceiptActionsList {
    fn from(v: Vec<ReceiptAction>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ReceiptAction> for ReceiptActionsList {
    fn from_iter<I: IntoIterator<Item = ReceiptAction>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ReceiptAction>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlReceiptActionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ReceiptAction>,
}

impl From<Vec<ReceiptAction>> for XmlReceiptActionList {
    fn from(v: Vec<ReceiptAction>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ReceiptAction> for XmlReceiptActionList {
    fn from_iter<I: IntoIterator<Item = ReceiptAction>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReceiptAction")]
pub struct ReceiptAction {
    #[serde(rename = "AddHeaderAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_header_action: Option<AddHeaderAction>,
    #[serde(rename = "BounceAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounce_action: Option<BounceAction>,
    #[serde(rename = "ConnectAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_action: Option<ConnectAction>,
    #[serde(rename = "LambdaAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_action: Option<LambdaAction>,
    #[serde(rename = "S3Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_action: Option<S3Action>,
    #[serde(rename = "SNSAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_n_s_action: Option<SNSAction>,
    #[serde(rename = "StopAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_action: Option<StopAction>,
    #[serde(rename = "WorkmailAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workmail_action: Option<WorkmailAction>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AddHeaderAction")]
pub struct AddHeaderAction {
    #[serde(rename = "HeaderName")]
    #[serde(default)]
    pub header_name: String,
    #[serde(rename = "HeaderValue")]
    #[serde(default)]
    pub header_value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BounceAction")]
pub struct BounceAction {
    #[serde(rename = "Message")]
    #[serde(default)]
    pub message: String,
    #[serde(rename = "Sender")]
    #[serde(default)]
    pub sender: String,
    #[serde(rename = "SmtpReplyCode")]
    #[serde(default)]
    pub smtp_reply_code: String,
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ConnectAction")]
pub struct ConnectAction {
    #[serde(rename = "IAMRoleARN")]
    #[serde(default)]
    pub i_a_m_role_a_r_n: String,
    #[serde(rename = "InstanceARN")]
    #[serde(default)]
    pub instance_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LambdaAction")]
pub struct LambdaAction {
    #[serde(rename = "FunctionArn")]
    #[serde(default)]
    pub function_arn: String,
    #[serde(rename = "InvocationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_type: Option<String>,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3Action")]
pub struct S3Action {
    #[serde(rename = "BucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "ObjectKeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_key_prefix: Option<String>,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SNSAction")]
pub struct SNSAction {
    #[serde(rename = "Encoding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    pub topic_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StopAction")]
pub struct StopAction {
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "WorkmailAction")]
pub struct WorkmailAction {
    #[serde(rename = "OrganizationArn")]
    #[serde(default)]
    pub organization_arn: String,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecipientsList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for RecipientsList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for RecipientsList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConfigurationSetEventDestinationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteTemplateRequest")]
pub struct DeleteTemplateRequest {
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReorderReceiptRuleSetRequest")]
pub struct ReorderReceiptRuleSetRequest {
    #[serde(rename = "RuleNames")]
    #[serde(default)]
    pub rule_names: ReceiptRuleNamesList,
    #[serde(rename = "RuleSetName")]
    #[serde(default)]
    pub rule_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReceiptRuleNamesList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ReceiptRuleNamesList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ReceiptRuleNamesList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetActiveReceiptRuleSetRequest")]
pub struct SetActiveReceiptRuleSetRequest {
    #[serde(rename = "RuleSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateConfigurationSetSendingEnabledRequest")]
pub struct UpdateConfigurationSetSendingEnabledRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    pub configuration_set_name: String,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateCustomVerificationEmailTemplateRequest")]
pub struct UpdateCustomVerificationEmailTemplateRequest {
    #[serde(rename = "FailureRedirectionURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_redirection_u_r_l: Option<String>,
    #[serde(rename = "FromEmailAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_email_address: Option<String>,
    #[serde(rename = "SuccessRedirectionURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_redirection_u_r_l: Option<String>,
    #[serde(rename = "TemplateContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_content: Option<String>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
    #[serde(rename = "TemplateSubject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_subject: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteConfigurationSetRequest")]
pub struct DeleteConfigurationSetRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    pub configuration_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "VerifyDomainDkimRequest")]
pub struct VerifyDomainDkimRequest {
    #[serde(rename = "Domain")]
    #[serde(default)]
    pub domain: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetIdentityMailFromDomainRequest")]
pub struct SetIdentityMailFromDomainRequest {
    #[serde(rename = "BehaviorOnMXFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior_on_m_x_failure: Option<String>,
    #[serde(rename = "Identity")]
    #[serde(default)]
    pub identity: String,
    #[serde(rename = "MailFromDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_from_domain: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateReceiptRuleSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateReceiptFilterResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetIdentityDkimEnabledResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeActiveReceiptRuleSetResult")]
pub struct DescribeActiveReceiptRuleSetResponse {
    #[serde(rename = "Metadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ReceiptRuleSetMetadata>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<ReceiptRulesList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetIdentityDkimAttributesRequest")]
pub struct GetIdentityDkimAttributesRequest {
    #[serde(rename = "Identities")]
    #[serde(default)]
    pub identities: IdentityList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateTemplateRequest")]
pub struct UpdateTemplateRequest {
    #[serde(rename = "Template")]
    #[serde(default)]
    pub template: Template,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Template")]
pub struct Template {
    #[serde(rename = "HtmlPart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_part: Option<String>,
    #[serde(rename = "SubjectPart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_part: Option<String>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
    #[serde(rename = "TextPart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_part: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateReceiptRuleSetRequest")]
pub struct CreateReceiptRuleSetRequest {
    #[serde(rename = "RuleSetName")]
    #[serde(default)]
    pub rule_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccountSendingEnabledResult")]
pub struct GetAccountSendingEnabledResponse {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SendCustomVerificationEmailRequest")]
pub struct SendCustomVerificationEmailRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set_name: Option<String>,
    #[serde(rename = "EmailAddress")]
    #[serde(default)]
    pub email_address: String,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConfigurationSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateReceiptRuleResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SendEmailRequest")]
pub struct SendEmailRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set_name: Option<String>,
    #[serde(rename = "Destination")]
    #[serde(default)]
    pub destination: Destination,
    #[serde(rename = "Message")]
    #[serde(default)]
    pub message: Message,
    #[serde(rename = "ReplyToAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_addresses: Option<AddressList>,
    #[serde(rename = "ReturnPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_path: Option<String>,
    #[serde(rename = "ReturnPathArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_path_arn: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: String,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<MessageTagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Destination")]
pub struct Destination {
    #[serde(rename = "BccAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc_addresses: Option<AddressList>,
    #[serde(rename = "CcAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_addresses: Option<AddressList>,
    #[serde(rename = "ToAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_addresses: Option<AddressList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddressList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AddressList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AddressList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Message")]
pub struct Message {
    #[serde(rename = "Body")]
    #[serde(default)]
    pub body: Body,
    #[serde(rename = "Subject")]
    #[serde(default)]
    pub subject: Content,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Body")]
pub struct Body {
    #[serde(rename = "Html")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<Content>,
    #[serde(rename = "Text")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Content>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Content")]
pub struct Content {
    #[serde(rename = "Charset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charset: Option<String>,
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MessageTagList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<MessageTag>,
}
impl From<Vec<MessageTag>> for MessageTagList {
    fn from(v: Vec<MessageTag>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<MessageTag> for MessageTagList {
    fn from_iter<I: IntoIterator<Item = MessageTag>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<MessageTag>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlMessageTagList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<MessageTag>,
}

impl From<Vec<MessageTag>> for XmlMessageTagList {
    fn from(v: Vec<MessageTag>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<MessageTag> for XmlMessageTagList {
    fn from_iter<I: IntoIterator<Item = MessageTag>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MessageTag")]
pub struct MessageTag {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListReceiptFiltersRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListVerifiedEmailAddressesResult")]
pub struct ListVerifiedEmailAddressesResponse {
    #[serde(rename = "VerifiedEmailAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_email_addresses: Option<AddressList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteReceiptRuleRequest")]
pub struct DeleteReceiptRuleRequest {
    #[serde(rename = "RuleName")]
    #[serde(default)]
    pub rule_name: String,
    #[serde(rename = "RuleSetName")]
    #[serde(default)]
    pub rule_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SendBulkTemplatedEmailRequest")]
pub struct SendBulkTemplatedEmailRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set_name: Option<String>,
    #[serde(rename = "DefaultTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tags: Option<MessageTagList>,
    #[serde(rename = "DefaultTemplateData")]
    #[serde(default)]
    pub default_template_data: String,
    #[serde(rename = "Destinations")]
    #[serde(default)]
    pub destinations: BulkEmailDestinationList,
    #[serde(rename = "ReplyToAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_addresses: Option<AddressList>,
    #[serde(rename = "ReturnPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_path: Option<String>,
    #[serde(rename = "ReturnPathArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_path_arn: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: String,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    #[serde(rename = "Template")]
    #[serde(default)]
    pub template: String,
    #[serde(rename = "TemplateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BulkEmailDestinationList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<BulkEmailDestination>,
}
impl From<Vec<BulkEmailDestination>> for BulkEmailDestinationList {
    fn from(v: Vec<BulkEmailDestination>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<BulkEmailDestination> for BulkEmailDestinationList {
    fn from_iter<I: IntoIterator<Item = BulkEmailDestination>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<BulkEmailDestination>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlBulkEmailDestinationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<BulkEmailDestination>,
}

impl From<Vec<BulkEmailDestination>> for XmlBulkEmailDestinationList {
    fn from(v: Vec<BulkEmailDestination>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<BulkEmailDestination> for XmlBulkEmailDestinationList {
    fn from_iter<I: IntoIterator<Item = BulkEmailDestination>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BulkEmailDestination")]
pub struct BulkEmailDestination {
    #[serde(rename = "Destination")]
    #[serde(default)]
    pub destination: Destination,
    #[serde(rename = "ReplacementTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_tags: Option<MessageTagList>,
    #[serde(rename = "ReplacementTemplateData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_template_data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConfigurationSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeConfigurationSetRequest")]
pub struct DescribeConfigurationSetRequest {
    #[serde(rename = "ConfigurationSetAttributeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set_attribute_names: Option<ConfigurationSetAttributeList>,
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    pub configuration_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationSetAttributeList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ConfigurationSetAttributeList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ConfigurationSetAttributeList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTemplateResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReceiptRuleSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetCustomVerificationEmailTemplateRequest")]
pub struct GetCustomVerificationEmailTemplateRequest {
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetSendStatisticsResult")]
pub struct GetSendStatisticsResponse {
    #[serde(rename = "SendDataPoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_data_points: Option<SendDataPointList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendDataPointList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<SendDataPoint>,
}
impl From<Vec<SendDataPoint>> for SendDataPointList {
    fn from(v: Vec<SendDataPoint>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<SendDataPoint> for SendDataPointList {
    fn from_iter<I: IntoIterator<Item = SendDataPoint>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<SendDataPoint>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSendDataPointList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<SendDataPoint>,
}

impl From<Vec<SendDataPoint>> for XmlSendDataPointList {
    fn from(v: Vec<SendDataPoint>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<SendDataPoint> for XmlSendDataPointList {
    fn from_iter<I: IntoIterator<Item = SendDataPoint>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SendDataPoint")]
pub struct SendDataPoint {
    #[serde(rename = "Bounces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounces: Option<i64>,
    #[serde(rename = "Complaints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complaints: Option<i64>,
    #[serde(rename = "DeliveryAttempts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_attempts: Option<i64>,
    #[serde(rename = "Rejects")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejects: Option<i64>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutIdentityPolicyRequest")]
pub struct PutIdentityPolicyRequest {
    #[serde(rename = "Identity")]
    #[serde(default)]
    pub identity: String,
    #[serde(rename = "Policy")]
    #[serde(default)]
    pub policy: String,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutConfigurationSetDeliveryOptionsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeReceiptRuleResult")]
pub struct DescribeReceiptRuleResponse {
    #[serde(rename = "Rule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<ReceiptRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetIdentityHeadersInNotificationsEnabledResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateConfigurationSetReputationMetricsEnabledRequest")]
pub struct UpdateConfigurationSetReputationMetricsEnabledRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    pub configuration_set_name: String,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CloneReceiptRuleSetRequest")]
pub struct CloneReceiptRuleSetRequest {
    #[serde(rename = "OriginalRuleSetName")]
    #[serde(default)]
    pub original_rule_set_name: String,
    #[serde(rename = "RuleSetName")]
    #[serde(default)]
    pub rule_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateConfigurationSetEventDestinationRequest")]
pub struct UpdateConfigurationSetEventDestinationRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    pub configuration_set_name: String,
    #[serde(rename = "EventDestination")]
    #[serde(default)]
    pub event_destination: EventDestination,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SendRawEmailRequest")]
pub struct SendRawEmailRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set_name: Option<String>,
    #[serde(rename = "Destinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<AddressList>,
    #[serde(rename = "FromArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_arn: Option<String>,
    #[serde(rename = "RawMessage")]
    #[serde(default)]
    pub raw_message: RawMessage,
    #[serde(rename = "ReturnPathArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_path_arn: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<MessageTagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RawMessage")]
pub struct RawMessage {
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListCustomVerificationEmailTemplatesResult")]
pub struct ListCustomVerificationEmailTemplatesResponse {
    #[serde(rename = "CustomVerificationEmailTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_verification_email_templates: Option<CustomVerificationEmailTemplates>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomVerificationEmailTemplates {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CustomVerificationEmailTemplate>,
}
impl From<Vec<CustomVerificationEmailTemplate>> for CustomVerificationEmailTemplates {
    fn from(v: Vec<CustomVerificationEmailTemplate>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<CustomVerificationEmailTemplate> for CustomVerificationEmailTemplates {
    fn from_iter<I: IntoIterator<Item = CustomVerificationEmailTemplate>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<CustomVerificationEmailTemplate>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlCustomVerificationEmailTemplateList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<CustomVerificationEmailTemplate>,
}

impl From<Vec<CustomVerificationEmailTemplate>> for XmlCustomVerificationEmailTemplateList {
    fn from(v: Vec<CustomVerificationEmailTemplate>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<CustomVerificationEmailTemplate> for XmlCustomVerificationEmailTemplateList {
    fn from_iter<I: IntoIterator<Item = CustomVerificationEmailTemplate>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CustomVerificationEmailTemplate")]
pub struct CustomVerificationEmailTemplate {
    #[serde(rename = "FailureRedirectionURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_redirection_u_r_l: Option<String>,
    #[serde(rename = "FromEmailAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_email_address: Option<String>,
    #[serde(rename = "SuccessRedirectionURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_redirection_u_r_l: Option<String>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(rename = "TemplateSubject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_subject: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SendTemplatedEmailResult")]
pub struct SendTemplatedEmailResponse {
    #[serde(rename = "MessageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeReceiptRuleSetRequest")]
pub struct DescribeReceiptRuleSetRequest {
    #[serde(rename = "RuleSetName")]
    #[serde(default)]
    pub rule_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetIdentityNotificationTopicRequest")]
pub struct SetIdentityNotificationTopicRequest {
    #[serde(rename = "Identity")]
    #[serde(default)]
    pub identity: String,
    #[serde(rename = "NotificationType")]
    #[serde(default)]
    pub notification_type: String,
    #[serde(rename = "SnsTopic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateConfigurationSetTrackingOptionsRequest")]
pub struct CreateConfigurationSetTrackingOptionsRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    pub configuration_set_name: String,
    #[serde(rename = "TrackingOptions")]
    #[serde(default)]
    pub tracking_options: TrackingOptions,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TrackingOptions")]
pub struct TrackingOptions {
    #[serde(rename = "CustomRedirectDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_redirect_domain: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListConfigurationSetsRequest")]
pub struct ListConfigurationSetsRequest {
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteConfigurationSetEventDestinationRequest")]
pub struct DeleteConfigurationSetEventDestinationRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    pub configuration_set_name: String,
    #[serde(rename = "EventDestinationName")]
    #[serde(default)]
    pub event_destination_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "VerifyEmailIdentityRequest")]
pub struct VerifyEmailIdentityRequest {
    #[serde(rename = "EmailAddress")]
    #[serde(default)]
    pub email_address: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListIdentityPoliciesResult")]
pub struct ListIdentityPoliciesResponse {
    #[serde(rename = "PolicyNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_names: Option<PolicyNameList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PolicyNameList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for PolicyNameList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for PolicyNameList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "VerifyDomainDkimResult")]
pub struct VerifyDomainDkimResponse {
    #[serde(rename = "DkimTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkim_tokens: Option<VerificationTokenList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VerificationTokenList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for VerificationTokenList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for VerificationTokenList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteReceiptRuleSetRequest")]
pub struct DeleteReceiptRuleSetRequest {
    #[serde(rename = "RuleSetName")]
    #[serde(default)]
    pub rule_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeConfigurationSetResult")]
pub struct DescribeConfigurationSetResponse {
    #[serde(rename = "ConfigurationSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set: Option<ConfigurationSet>,
    #[serde(rename = "DeliveryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_options: Option<DeliveryOptions>,
    #[serde(rename = "EventDestinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_destinations: Option<EventDestinations>,
    #[serde(rename = "ReputationOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reputation_options: Option<ReputationOptions>,
    #[serde(rename = "TrackingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_options: Option<TrackingOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeliveryOptions")]
pub struct DeliveryOptions {
    #[serde(rename = "TlsPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventDestinations {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<EventDestination>,
}
impl From<Vec<EventDestination>> for EventDestinations {
    fn from(v: Vec<EventDestination>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<EventDestination> for EventDestinations {
    fn from_iter<I: IntoIterator<Item = EventDestination>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<EventDestination>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlEventDestinationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<EventDestination>,
}

impl From<Vec<EventDestination>> for XmlEventDestinationList {
    fn from(v: Vec<EventDestination>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<EventDestination> for XmlEventDestinationList {
    fn from_iter<I: IntoIterator<Item = EventDestination>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReputationOptions")]
pub struct ReputationOptions {
    #[serde(rename = "LastFreshStart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_fresh_start: Option<String>,
    #[serde(rename = "ReputationMetricsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reputation_metrics_enabled: Option<bool>,
    #[serde(rename = "SendingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TestRenderTemplateRequest")]
pub struct TestRenderTemplateRequest {
    #[serde(rename = "TemplateData")]
    #[serde(default)]
    pub template_data: String,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteCustomVerificationEmailTemplateRequest")]
pub struct DeleteCustomVerificationEmailTemplateRequest {
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeReceiptRuleRequest")]
pub struct DescribeReceiptRuleRequest {
    #[serde(rename = "RuleName")]
    #[serde(default)]
    pub rule_name: String,
    #[serde(rename = "RuleSetName")]
    #[serde(default)]
    pub rule_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTemplatesResult")]
pub struct ListTemplatesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TemplatesMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templates_metadata: Option<TemplateMetadataList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TemplateMetadataList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TemplateMetadata>,
}
impl From<Vec<TemplateMetadata>> for TemplateMetadataList {
    fn from(v: Vec<TemplateMetadata>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TemplateMetadata> for TemplateMetadataList {
    fn from_iter<I: IntoIterator<Item = TemplateMetadata>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TemplateMetadata>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTemplateMetadataList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TemplateMetadata>,
}

impl From<Vec<TemplateMetadata>> for XmlTemplateMetadataList {
    fn from(v: Vec<TemplateMetadata>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TemplateMetadata> for XmlTemplateMetadataList {
    fn from_iter<I: IntoIterator<Item = TemplateMetadata>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TemplateMetadata")]
pub struct TemplateMetadata {
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateReceiptRuleRequest")]
pub struct CreateReceiptRuleRequest {
    #[serde(rename = "After")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(rename = "Rule")]
    #[serde(default)]
    pub rule: ReceiptRule,
    #[serde(rename = "RuleSetName")]
    #[serde(default)]
    pub rule_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteVerifiedEmailAddressRequest")]
pub struct DeleteVerifiedEmailAddressRequest {
    #[serde(rename = "EmailAddress")]
    #[serde(default)]
    pub email_address: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetTemplateRequest")]
pub struct GetTemplateRequest {
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutIdentityPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetActiveReceiptRuleSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetIdentityNotificationTopicResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetReceiptRulePositionRequest")]
pub struct SetReceiptRulePositionRequest {
    #[serde(rename = "After")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(rename = "RuleName")]
    #[serde(default)]
    pub rule_name: String,
    #[serde(rename = "RuleSetName")]
    #[serde(default)]
    pub rule_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetIdentityPoliciesRequest")]
pub struct GetIdentityPoliciesRequest {
    #[serde(rename = "Identity")]
    #[serde(default)]
    pub identity: String,
    #[serde(rename = "PolicyNames")]
    #[serde(default)]
    pub policy_names: PolicyNameList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SendBulkTemplatedEmailResult")]
pub struct SendBulkTemplatedEmailResponse {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<BulkEmailDestinationStatusList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BulkEmailDestinationStatusList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<BulkEmailDestinationStatus>,
}
impl From<Vec<BulkEmailDestinationStatus>> for BulkEmailDestinationStatusList {
    fn from(v: Vec<BulkEmailDestinationStatus>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<BulkEmailDestinationStatus> for BulkEmailDestinationStatusList {
    fn from_iter<I: IntoIterator<Item = BulkEmailDestinationStatus>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<BulkEmailDestinationStatus>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlBulkEmailDestinationStatusList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<BulkEmailDestinationStatus>,
}

impl From<Vec<BulkEmailDestinationStatus>> for XmlBulkEmailDestinationStatusList {
    fn from(v: Vec<BulkEmailDestinationStatus>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<BulkEmailDestinationStatus> for XmlBulkEmailDestinationStatusList {
    fn from_iter<I: IntoIterator<Item = BulkEmailDestinationStatus>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BulkEmailDestinationStatus")]
pub struct BulkEmailDestinationStatus {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "MessageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReceiptRuleResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SendCustomVerificationEmailResult")]
pub struct SendCustomVerificationEmailResponse {
    #[serde(rename = "MessageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VerifyEmailIdentityResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetTemplateResult")]
pub struct GetTemplateResponse {
    #[serde(rename = "Template")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<Template>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListReceiptFiltersResult")]
pub struct ListReceiptFiltersResponse {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ReceiptFilterList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReceiptFilterList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ReceiptFilter>,
}
impl From<Vec<ReceiptFilter>> for ReceiptFilterList {
    fn from(v: Vec<ReceiptFilter>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ReceiptFilter> for ReceiptFilterList {
    fn from_iter<I: IntoIterator<Item = ReceiptFilter>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ReceiptFilter>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlReceiptFilterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ReceiptFilter>,
}

impl From<Vec<ReceiptFilter>> for XmlReceiptFilterList {
    fn from(v: Vec<ReceiptFilter>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ReceiptFilter> for XmlReceiptFilterList {
    fn from_iter<I: IntoIterator<Item = ReceiptFilter>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReceiptFilter")]
pub struct ReceiptFilter {
    #[serde(rename = "IpFilter")]
    #[serde(default)]
    pub ip_filter: ReceiptIpFilter,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReceiptIpFilter")]
pub struct ReceiptIpFilter {
    #[serde(rename = "Cidr")]
    #[serde(default)]
    pub cidr: String,
    #[serde(rename = "Policy")]
    #[serde(default)]
    pub policy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIdentityPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetCustomVerificationEmailTemplateResult")]
pub struct GetCustomVerificationEmailTemplateResponse {
    #[serde(rename = "FailureRedirectionURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_redirection_u_r_l: Option<String>,
    #[serde(rename = "FromEmailAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_email_address: Option<String>,
    #[serde(rename = "SuccessRedirectionURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_redirection_u_r_l: Option<String>,
    #[serde(rename = "TemplateContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_content: Option<String>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(rename = "TemplateSubject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_subject: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetIdentityDkimEnabledRequest")]
pub struct SetIdentityDkimEnabledRequest {
    #[serde(rename = "DkimEnabled")]
    #[serde(default)]
    pub dkim_enabled: bool,
    #[serde(rename = "Identity")]
    #[serde(default)]
    pub identity: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateReceiptRuleRequest")]
pub struct UpdateReceiptRuleRequest {
    #[serde(rename = "Rule")]
    #[serde(default)]
    pub rule: ReceiptRule,
    #[serde(rename = "RuleSetName")]
    #[serde(default)]
    pub rule_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SendBounceResult")]
pub struct SendBounceResponse {
    #[serde(rename = "MessageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListCustomVerificationEmailTemplatesRequest")]
pub struct ListCustomVerificationEmailTemplatesRequest {
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
pub struct CreateConfigurationSetTrackingOptionsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteConfigurationSetTrackingOptionsRequest")]
pub struct DeleteConfigurationSetTrackingOptionsRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    pub configuration_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetIdentityDkimAttributesResult")]
pub struct GetIdentityDkimAttributesResponse {
    #[serde(rename = "DkimAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkim_attributes: Option<std::collections::HashMap<String, IdentityDkimAttributes>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "IdentityDkimAttributes")]
pub struct IdentityDkimAttributes {
    #[serde(rename = "DkimEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkim_enabled: Option<bool>,
    #[serde(rename = "DkimTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkim_tokens: Option<VerificationTokenList>,
    #[serde(rename = "DkimVerificationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkim_verification_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateConfigurationSetRequest")]
pub struct CreateConfigurationSetRequest {
    #[serde(rename = "ConfigurationSet")]
    #[serde(default)]
    pub configuration_set: ConfigurationSet,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SendBounceRequest")]
pub struct SendBounceRequest {
    #[serde(rename = "BounceSender")]
    #[serde(default)]
    pub bounce_sender: String,
    #[serde(rename = "BounceSenderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounce_sender_arn: Option<String>,
    #[serde(rename = "BouncedRecipientInfoList")]
    #[serde(default)]
    pub bounced_recipient_info_list: BouncedRecipientInfoList,
    #[serde(rename = "Explanation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[serde(rename = "MessageDsn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_dsn: Option<MessageDsn>,
    #[serde(rename = "OriginalMessageId")]
    #[serde(default)]
    pub original_message_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BouncedRecipientInfoList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<BouncedRecipientInfo>,
}
impl From<Vec<BouncedRecipientInfo>> for BouncedRecipientInfoList {
    fn from(v: Vec<BouncedRecipientInfo>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<BouncedRecipientInfo> for BouncedRecipientInfoList {
    fn from_iter<I: IntoIterator<Item = BouncedRecipientInfo>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<BouncedRecipientInfo>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlBouncedRecipientInfoList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<BouncedRecipientInfo>,
}

impl From<Vec<BouncedRecipientInfo>> for XmlBouncedRecipientInfoList {
    fn from(v: Vec<BouncedRecipientInfo>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<BouncedRecipientInfo> for XmlBouncedRecipientInfoList {
    fn from_iter<I: IntoIterator<Item = BouncedRecipientInfo>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BouncedRecipientInfo")]
pub struct BouncedRecipientInfo {
    #[serde(rename = "BounceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounce_type: Option<String>,
    #[serde(rename = "Recipient")]
    #[serde(default)]
    pub recipient: String,
    #[serde(rename = "RecipientArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_arn: Option<String>,
    #[serde(rename = "RecipientDsnFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_dsn_fields: Option<RecipientDsnFields>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RecipientDsnFields")]
pub struct RecipientDsnFields {
    #[serde(rename = "Action")]
    #[serde(default)]
    pub action: String,
    #[serde(rename = "DiagnosticCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostic_code: Option<String>,
    #[serde(rename = "ExtensionFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_fields: Option<ExtensionFieldList>,
    #[serde(rename = "FinalRecipient")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_recipient: Option<String>,
    #[serde(rename = "LastAttemptDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_attempt_date: Option<String>,
    #[serde(rename = "RemoteMta")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_mta: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExtensionFieldList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ExtensionField>,
}
impl From<Vec<ExtensionField>> for ExtensionFieldList {
    fn from(v: Vec<ExtensionField>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ExtensionField> for ExtensionFieldList {
    fn from_iter<I: IntoIterator<Item = ExtensionField>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ExtensionField>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlExtensionFieldList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ExtensionField>,
}

impl From<Vec<ExtensionField>> for XmlExtensionFieldList {
    fn from(v: Vec<ExtensionField>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ExtensionField> for XmlExtensionFieldList {
    fn from_iter<I: IntoIterator<Item = ExtensionField>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ExtensionField")]
pub struct ExtensionField {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MessageDsn")]
pub struct MessageDsn {
    #[serde(rename = "ArrivalDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrival_date: Option<String>,
    #[serde(rename = "ExtensionFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_fields: Option<ExtensionFieldList>,
    #[serde(rename = "ReportingMta")]
    #[serde(default)]
    pub reporting_mta: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloneReceiptRuleSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetIdentityVerificationAttributesRequest")]
pub struct GetIdentityVerificationAttributesRequest {
    #[serde(rename = "Identities")]
    #[serde(default)]
    pub identities: IdentityList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateTemplateRequest")]
pub struct CreateTemplateRequest {
    #[serde(rename = "Template")]
    #[serde(default)]
    pub template: Template,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateReceiptFilterRequest")]
pub struct CreateReceiptFilterRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    pub filter: ReceiptFilter,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutConfigurationSetDeliveryOptionsRequest")]
pub struct PutConfigurationSetDeliveryOptionsRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    pub configuration_set_name: String,
    #[serde(rename = "DeliveryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_options: Option<DeliveryOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SendTemplatedEmailRequest")]
pub struct SendTemplatedEmailRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set_name: Option<String>,
    #[serde(rename = "Destination")]
    #[serde(default)]
    pub destination: Destination,
    #[serde(rename = "ReplyToAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_addresses: Option<AddressList>,
    #[serde(rename = "ReturnPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_path: Option<String>,
    #[serde(rename = "ReturnPathArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_path_arn: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: String,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<MessageTagList>,
    #[serde(rename = "Template")]
    #[serde(default)]
    pub template: String,
    #[serde(rename = "TemplateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_arn: Option<String>,
    #[serde(rename = "TemplateData")]
    #[serde(default)]
    pub template_data: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateConfigurationSetTrackingOptionsRequest")]
pub struct UpdateConfigurationSetTrackingOptionsRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    pub configuration_set_name: String,
    #[serde(rename = "TrackingOptions")]
    #[serde(default)]
    pub tracking_options: TrackingOptions,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTemplateResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TestRenderTemplateResult")]
pub struct TestRenderTemplateResponse {
    #[serde(rename = "RenderedTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendered_template: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeActiveReceiptRuleSetRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTemplatesRequest")]
pub struct ListTemplatesRequest {
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SendRawEmailResult")]
pub struct SendRawEmailResponse {
    #[serde(rename = "MessageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConfigurationSetEventDestinationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteReceiptFilterRequest")]
pub struct DeleteReceiptFilterRequest {
    #[serde(rename = "FilterName")]
    #[serde(default)]
    pub filter_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetReceiptRulePositionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "VerifyDomainIdentityRequest")]
pub struct VerifyDomainIdentityRequest {
    #[serde(rename = "Domain")]
    #[serde(default)]
    pub domain: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConfigurationSetTrackingOptionsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateReceiptRuleResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListReceiptRuleSetsRequest")]
pub struct ListReceiptRuleSetsRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReceiptFilterResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetSendQuotaResult")]
pub struct GetSendQuotaResponse {
    #[serde(rename = "Max24HourSend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max24_hour_send: Option<f64>,
    #[serde(rename = "MaxSendRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_send_rate: Option<f64>,
    #[serde(rename = "SentLast24Hours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_last24_hours: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConfigurationSetEventDestinationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListIdentitiesRequest")]
pub struct ListIdentitiesRequest {
    #[serde(rename = "IdentityType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_type: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListIdentityPoliciesRequest")]
pub struct ListIdentityPoliciesRequest {
    #[serde(rename = "Identity")]
    #[serde(default)]
    pub identity: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetIdentityHeadersInNotificationsEnabledRequest")]
pub struct SetIdentityHeadersInNotificationsEnabledRequest {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "Identity")]
    #[serde(default)]
    pub identity: String,
    #[serde(rename = "NotificationType")]
    #[serde(default)]
    pub notification_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "VerifyDomainIdentityResult")]
pub struct VerifyDomainIdentityResponse {
    #[serde(rename = "VerificationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_token: Option<String>,
}
