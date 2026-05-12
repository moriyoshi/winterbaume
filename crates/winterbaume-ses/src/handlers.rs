//! SES v1 service handler (awsQuery protocol, 2010-12-01).
//!
//! SES v1 uses the `email.{region}.amazonaws.com` endpoint (same as SES v2).
//! This handler is registered AFTER winterbaume-ses so it is only reached
//! for requests that the SES v2 handler passes on (i.e., non-/v2/ paths).
//!
//! The awsQuery protocol: every request is an HTTP POST to `/` (or `/`),
//! body is URL-encoded form data, dispatch on the `Action` field.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::state::{SesV1Error, SesV1State};
use crate::types::{EventDestination, ReceiptRule, Template};
use crate::views::SesV1StateView;
use crate::wire;

/// SES v1 service handler.
pub struct SesService {
    pub(crate) state: Arc<BackendState<SesV1State>>,
    pub(crate) notifier: StateChangeNotifier<SesV1StateView>,
}

impl SesService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for SesService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for SesService {
    fn service_name(&self) -> &str {
        "email"
    }

    fn url_patterns(&self) -> Vec<&str> {
        // SES v1 handles the ses.*.amazonaws.com endpoint (secondary/legacy endpoint).
        // The primary email.*.amazonaws.com endpoint is handled by winterbaume-ses (SES v2).
        // In standalone tests, clients are configured to use the ses.* endpoint explicitly.
        vec![
            r"https?://ses\..*\.amazonaws\.com",
            r"https?://ses\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl SesService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        // Parse form-encoded body (awsQuery protocol)
        let body_str = std::str::from_utf8(&request.body).unwrap_or("");
        let params = parse_query_string(body_str);

        let action = match params.get("Action") {
            Some(a) => a.as_str(),
            None => {
                // Not an awsQuery request — pass through (SES v2 uses JSON, not form data)
                return MockResponse::error(400, "MissingAction", "Missing 'Action' parameter");
            }
        };

        let state = self.state.get(account_id, &region);

        let mutating_actions = [
            "VerifyEmailAddress",
            "VerifyEmailIdentity",
            "VerifyDomainIdentity",
            "DeleteIdentity",
            "SetIdentityFeedbackForwardingEnabled",
            "SetIdentityMailFromDomain",
            "SetIdentityNotificationTopic",
            "CreateConfigurationSet",
            "DeleteConfigurationSet",
            "CreateConfigurationSetEventDestination",
            "CreateReceiptRuleSet",
            "DeleteReceiptRuleSet",
            "CloneReceiptRuleSet",
            "SetActiveReceiptRuleSet",
            "CreateReceiptRule",
            "UpdateReceiptRule",
            "CreateTemplate",
            "DeleteTemplate",
            "UpdateTemplate",
            "UpdateConfigurationSetReputationMetricsEnabled",
            "SendEmail",
            "SendRawEmail",
            "SendTemplatedEmail",
            "SendBulkTemplatedEmail",
        ];

        let response = match action {
            "VerifyEmailAddress" => self.handle_verify_email_address(&state, &params).await,
            "VerifyEmailIdentity" => self.handle_verify_email_identity(&state, &params).await,
            "VerifyDomainIdentity" => self.handle_verify_domain_identity(&state, &params).await,
            "DeleteIdentity" => self.handle_delete_identity(&state, &params).await,
            "ListIdentities" => self.handle_list_identities(&state, &params).await,
            "ListVerifiedEmailAddresses" => self.handle_list_verified_email_addresses(&state).await,
            "GetIdentityVerificationAttributes" => {
                self.handle_get_identity_verification_attributes(&state, &params)
                    .await
            }
            "GetIdentityDkimAttributes" => {
                self.handle_get_identity_dkim_attributes(&state, &params)
                    .await
            }
            "GetIdentityMailFromDomainAttributes" => {
                self.handle_get_identity_mail_from_domain_attributes(&state, &params)
                    .await
            }
            "GetIdentityNotificationAttributes" => {
                self.handle_get_identity_notification_attributes(&state, &params)
                    .await
            }
            "GetSendQuota" => self.handle_get_send_quota().await,
            "GetSendStatistics" => self.handle_get_send_statistics().await,
            "SetIdentityFeedbackForwardingEnabled" => {
                self.handle_set_identity_feedback_forwarding_enabled(&state, &params)
                    .await
            }
            "SetIdentityMailFromDomain" => {
                self.handle_set_identity_mail_from_domain(&state, &params)
                    .await
            }
            "SetIdentityNotificationTopic" => {
                self.handle_set_identity_notification_topic(&state, &params)
                    .await
            }
            "CreateConfigurationSet" => self.handle_create_configuration_set(&state, &params).await,
            "DeleteConfigurationSet" => self.handle_delete_configuration_set(&state, &params).await,
            "DescribeConfigurationSet" => {
                self.handle_describe_configuration_set(&state, &params)
                    .await
            }
            "ListConfigurationSets" => self.handle_list_configuration_sets(&state).await,
            "CreateConfigurationSetEventDestination" => {
                self.handle_create_configuration_set_event_destination(&state, &params)
                    .await
            }
            "CreateReceiptRuleSet" => self.handle_create_receipt_rule_set(&state, &params).await,
            "DeleteReceiptRuleSet" => self.handle_delete_receipt_rule_set(&state, &params).await,
            "CloneReceiptRuleSet" => self.handle_clone_receipt_rule_set(&state, &params).await,
            "SetActiveReceiptRuleSet" => {
                self.handle_set_active_receipt_rule_set(&state, &params)
                    .await
            }
            "DescribeActiveReceiptRuleSet" => {
                self.handle_describe_active_receipt_rule_set(&state).await
            }
            "DescribeReceiptRuleSet" => {
                self.handle_describe_receipt_rule_set(&state, &params).await
            }
            "ListReceiptRuleSets" => self.handle_list_receipt_rule_sets(&state).await,
            "CreateReceiptRule" => self.handle_create_receipt_rule(&state, &params).await,
            "DescribeReceiptRule" => self.handle_describe_receipt_rule(&state, &params).await,
            "UpdateReceiptRule" => self.handle_update_receipt_rule(&state, &params).await,
            "CreateTemplate" => self.handle_create_template(&state, &params).await,
            "DeleteTemplate" => self.handle_delete_template(&state, &params).await,
            "GetTemplate" => self.handle_get_template(&state, &params).await,
            "ListTemplates" => self.handle_list_templates(&state).await,
            "UpdateTemplate" => self.handle_update_template(&state, &params).await,
            "UpdateConfigurationSetReputationMetricsEnabled" => {
                self.handle_update_configuration_set_reputation_metrics_enabled(&state, &params)
                    .await
            }
            "SendEmail" => self.handle_send_email(&state, &params).await,
            "SendRawEmail" => self.handle_send_raw_email(&params).await,
            "SendTemplatedEmail" => self.handle_send_templated_email(&params).await,
            "SendBulkTemplatedEmail" => self.handle_send_bulk_templated_email(&params).await,
            _ => MockResponse::error(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for SES"),
            ),
        };

        if mutating_actions.contains(&action) && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }

        response
    }

    // --- Identity operations ---

    async fn handle_verify_email_address(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_verify_email_address_request(params) {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        if input.email_address.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing EmailAddress");
        }
        let mut state = state.write().await;
        state.verify_email_address(&input.email_address);
        wire::serialize_verify_email_address_response()
    }

    async fn handle_verify_email_identity(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_verify_email_identity_request(params) {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        if input.email_address.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing EmailAddress");
        }
        let mut state = state.write().await;
        state.verify_email_identity(&input.email_address);
        wire::serialize_verify_email_identity_response(&wire::VerifyEmailIdentityResponse {})
    }

    async fn handle_verify_domain_identity(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_verify_domain_identity_request(params) {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        if input.domain.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing Domain");
        }
        let mut state = state.write().await;
        let token = state.verify_domain_identity(&input.domain);
        wire::serialize_verify_domain_identity_response(&wire::VerifyDomainIdentityResponse {
            verification_token: Some(token),
        })
    }

    async fn handle_delete_identity(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_identity_request(params) {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        if input.identity.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing Identity");
        }
        let mut state = state.write().await;
        match state.delete_identity(&input.identity) {
            Ok(_) => wire::serialize_delete_identity_response(&wire::DeleteIdentityResponse {}),
            Err(e) => ses_error_from(&e),
        }
    }

    async fn handle_list_identities(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_identities_request(params) {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        let state = state.read().await;
        let identities = state.list_identities(input.identity_type.as_deref());
        wire::serialize_list_identities_response(&wire::ListIdentitiesResponse {
            identities: Some(wire::IdentityList::from(identities)),
            next_token: None,
        })
    }

    async fn handle_list_verified_email_addresses(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
    ) -> MockResponse {
        let state = state.read().await;
        let emails = state.list_verified_email_addresses();
        wire::serialize_list_verified_email_addresses_response(
            &wire::ListVerifiedEmailAddressesResponse {
                verified_email_addresses: Some(wire::AddressList::from(emails)),
            },
        )
    }

    async fn handle_get_identity_verification_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_identity_verification_attributes_request(params) {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        let identities = input.identities.items;
        let state = state.read().await;
        let attrs = state.get_identity_verification_attributes(&identities);
        // awsQuery maps use <entry><key>K</key><value>V</value></entry> pairs.
        // quick_xml::se cannot serialise HashMap<String, T> into that format,
        // so we build the XML directly.
        let mut entries = String::new();
        for (identity, (status, token)) in &attrs {
            let key_xml = xml_escape(identity);
            let token_xml = if let Some(t) = token {
                format!("<VerificationToken>{}</VerificationToken>", xml_escape(t))
            } else {
                String::new()
            };
            entries.push_str(&format!(
                "<entry><key>{key_xml}</key><value>\
                 <VerificationStatus>{status}</VerificationStatus>\
                 {token_xml}\
                 </value></entry>"
            ));
        }
        ses_map_response(
            "GetIdentityVerificationAttributes",
            "VerificationAttributes",
            &entries,
        )
    }

    async fn handle_get_identity_dkim_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_identity_dkim_attributes_request(params) {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        let identities = input.identities.items;
        let state = state.read().await;
        // awsQuery maps use <entry><key>K</key><value>V</value></entry> pairs.
        let mut entries = String::new();
        for name in &identities {
            let (enabled, status, tokens) = if let Some(id) = state.identities.get(name) {
                (
                    id.dkim_enabled,
                    "Success".to_string(),
                    id.dkim_tokens.clone(),
                )
            } else {
                (false, "NotStarted".to_string(), vec![])
            };
            let key_xml = xml_escape(name);
            let tokens_xml: String = tokens
                .iter()
                .map(|t| format!("<member>{}</member>", xml_escape(t)))
                .collect();
            let dkim_tokens_xml = if tokens.is_empty() {
                String::new()
            } else {
                format!("<DkimTokens>{tokens_xml}</DkimTokens>")
            };
            entries.push_str(&format!(
                "<entry><key>{key_xml}</key><value>\
                 <DkimEnabled>{enabled}</DkimEnabled>\
                 <DkimVerificationStatus>{status}</DkimVerificationStatus>\
                 {dkim_tokens_xml}\
                 </value></entry>"
            ));
        }
        ses_map_response("GetIdentityDkimAttributes", "DkimAttributes", &entries)
    }

    async fn handle_get_identity_mail_from_domain_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_identity_mail_from_domain_attributes_request(params)
        {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        let identities = input.identities.items;
        let state = state.read().await;
        // awsQuery maps use <entry><key>K</key><value>V</value></entry> pairs.
        let mut entries = String::new();
        for name in &identities {
            let (domain, status) = if let Some(id) = state.identities.get(name) {
                (id.mail_from_domain.clone(), "Success".to_string())
            } else {
                (None, "Success".to_string())
            };
            let key_xml = xml_escape(name);
            let domain_xml = if let Some(d) = &domain {
                format!("<MailFromDomain>{}</MailFromDomain>", xml_escape(d))
            } else {
                String::new()
            };
            entries.push_str(&format!(
                "<entry><key>{key_xml}</key><value>\
                 <MailFromDomainStatus>{status}</MailFromDomainStatus>\
                 <BehaviorOnMXFailure>UseDefaultValue</BehaviorOnMXFailure>\
                 {domain_xml}\
                 </value></entry>"
            ));
        }
        ses_map_response(
            "GetIdentityMailFromDomainAttributes",
            "MailFromDomainAttributes",
            &entries,
        )
    }

    async fn handle_get_identity_notification_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_identity_notification_attributes_request(params) {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        let identities = input.identities.items;
        let state = state.read().await;
        // awsQuery maps use <entry><key>K</key><value>V</value></entry> pairs.
        let mut entries = String::new();
        for name in &identities {
            let (bounce, complaint, delivery, forwarding) =
                if let Some(id) = state.identities.get(name) {
                    (
                        id.bounce_topic.clone(),
                        id.complaint_topic.clone(),
                        id.delivery_topic.clone(),
                        id.forwarding_enabled,
                    )
                } else {
                    (None, None, None, true)
                };
            let key_xml = xml_escape(name);
            let bounce_xml = if let Some(t) = &bounce {
                format!("<BounceTopic>{}</BounceTopic>", xml_escape(t))
            } else {
                String::new()
            };
            let complaint_xml = if let Some(t) = &complaint {
                format!("<ComplaintTopic>{}</ComplaintTopic>", xml_escape(t))
            } else {
                String::new()
            };
            let delivery_xml = if let Some(t) = &delivery {
                format!("<DeliveryTopic>{}</DeliveryTopic>", xml_escape(t))
            } else {
                String::new()
            };
            entries.push_str(&format!(
                "<entry><key>{key_xml}</key><value>\
                 <ForwardingEnabled>{forwarding}</ForwardingEnabled>\
                 {bounce_xml}{complaint_xml}{delivery_xml}\
                 </value></entry>"
            ));
        }
        ses_map_response(
            "GetIdentityNotificationAttributes",
            "NotificationAttributes",
            &entries,
        )
    }

    // STUB[no-telemetry]: Send quota is driven by real AWS account limits; the mock
    //   returns plausible static values (50 000/day, 14/s, 0 sent).
    async fn handle_get_send_quota(&self) -> MockResponse {
        wire::serialize_get_send_quota_response(&wire::GetSendQuotaResponse {
            max24_hour_send: Some(50000.0),
            max_send_rate: Some(14.0),
            sent_last24_hours: Some(0.0),
        })
    }

    // STUB[no-telemetry]: Send statistics are derived from real delivery telemetry;
    //   the mock always returns an empty data-point list.
    async fn handle_get_send_statistics(&self) -> MockResponse {
        wire::serialize_get_send_statistics_response(&wire::GetSendStatisticsResponse {
            send_data_points: Some(wire::SendDataPointList::default()),
        })
    }

    async fn handle_set_identity_feedback_forwarding_enabled(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_set_identity_feedback_forwarding_enabled_request(params)
        {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        if input.identity.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing Identity");
        }
        // Pre-codegen default for ForwardingEnabled was true; the Smithy type is
        // a non-Option bool that defaults to false via serde, so honour the
        // historical default when the param is absent.
        let enabled = params
            .get("ForwardingEnabled")
            .map(|s| s == "true")
            .unwrap_or(true);
        let identity = input.identity;
        let mut state = state.write().await;
        match state.set_identity_feedback_forwarding_enabled(&identity, enabled) {
            Ok(_) => wire::serialize_set_identity_feedback_forwarding_enabled_response(
                &wire::SetIdentityFeedbackForwardingEnabledResponse {},
            ),
            Err(e) => ses_error_from(&e),
        }
    }

    async fn handle_set_identity_mail_from_domain(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_set_identity_mail_from_domain_request(params) {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        if input.identity.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing Identity");
        }
        let mut state = state.write().await;
        match state.set_identity_mail_from_domain(&input.identity, input.mail_from_domain) {
            Ok(_) => wire::serialize_set_identity_mail_from_domain_response(
                &wire::SetIdentityMailFromDomainResponse {},
            ),
            Err(e) => ses_error_from(&e),
        }
    }

    async fn handle_set_identity_notification_topic(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_set_identity_notification_topic_request(params) {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        if input.identity.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing Identity");
        }
        if input.notification_type.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing NotificationType");
        }
        let mut state = state.write().await;
        match state.set_identity_notification_topic(
            &input.identity,
            &input.notification_type,
            input.sns_topic,
        ) {
            Ok(_) => wire::serialize_set_identity_notification_topic_response(
                &wire::SetIdentityNotificationTopicResponse {},
            ),
            Err(e) => ses_error_from(&e),
        }
    }

    // --- Configuration Sets ---

    async fn handle_create_configuration_set(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_configuration_set_request(params) {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        if input.configuration_set.name.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing ConfigurationSet.Name");
        }
        let mut state = state.write().await;
        match state.create_configuration_set(&input.configuration_set.name) {
            Ok(_) => wire::serialize_create_configuration_set_response(
                &wire::CreateConfigurationSetResponse {},
            ),
            Err(e) => ses_error_from(&e),
        }
    }

    async fn handle_delete_configuration_set(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_configuration_set_request(params) {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        if input.configuration_set_name.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing ConfigurationSetName");
        }
        let mut state = state.write().await;
        match state.delete_configuration_set(&input.configuration_set_name) {
            Ok(_) => wire::serialize_delete_configuration_set_response(
                &wire::DeleteConfigurationSetResponse {},
            ),
            Err(e) => ses_error_from(&e),
        }
    }

    async fn handle_describe_configuration_set(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_configuration_set_request(params) {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        if input.configuration_set_name.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing ConfigurationSetName");
        }
        let state = state.read().await;
        match state.describe_configuration_set(&input.configuration_set_name) {
            Ok(cs) => {
                let event_destinations: Vec<wire::EventDestination> = cs
                    .event_destinations
                    .iter()
                    .map(|ed| wire::EventDestination {
                        name: ed.name.clone(),
                        enabled: Some(ed.enabled),
                        matching_event_types: wire::EventTypes::from(
                            ed.matching_event_types.clone(),
                        ),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_describe_configuration_set_response(
                    &wire::DescribeConfigurationSetResponse {
                        configuration_set: Some(wire::ConfigurationSet {
                            name: cs.name.clone(),
                        }),
                        event_destinations: if event_destinations.is_empty() {
                            None
                        } else {
                            Some(wire::EventDestinations::from(event_destinations))
                        },
                        ..Default::default()
                    },
                )
            }
            Err(e) => ses_error_from(&e),
        }
    }

    async fn handle_list_configuration_sets(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
    ) -> MockResponse {
        let state = state.read().await;
        let names = state.list_configuration_sets();
        let members: Vec<wire::ConfigurationSet> = names
            .into_iter()
            .map(|n| wire::ConfigurationSet { name: n })
            .collect();
        wire::serialize_list_configuration_sets_response(&wire::ListConfigurationSetsResponse {
            configuration_sets: Some(wire::ConfigurationSets::from(members)),
            next_token: None,
        })
    }

    async fn handle_create_configuration_set_event_destination(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_configuration_set_event_destination_request(params) {
                Ok(v) => v,
                Err(e) => return ses_error(400, "InvalidParameterValue", &e),
            };
        if input.configuration_set_name.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing ConfigurationSetName");
        }
        if input.event_destination.name.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing EventDestination.Name");
        }

        let destination = EventDestination {
            name: input.event_destination.name,
            enabled: input.event_destination.enabled.unwrap_or(false),
            matching_event_types: input.event_destination.matching_event_types.items,
        };
        let mut state = state.write().await;
        match state
            .create_configuration_set_event_destination(&input.configuration_set_name, destination)
        {
            Ok(_) => wire::serialize_create_configuration_set_event_destination_response(
                &wire::CreateConfigurationSetEventDestinationResponse {},
            ),
            Err(e) => ses_error_from(&e),
        }
    }

    // --- Receipt Rule Sets ---

    async fn handle_create_receipt_rule_set(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_receipt_rule_set_request(params) {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        if input.rule_set_name.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing RuleSetName");
        }
        let mut state = state.write().await;
        match state.create_receipt_rule_set(&input.rule_set_name) {
            Ok(_) => wire::serialize_create_receipt_rule_set_response(
                &wire::CreateReceiptRuleSetResponse {},
            ),
            Err(e) => ses_error_from(&e),
        }
    }

    async fn handle_delete_receipt_rule_set(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_receipt_rule_set_request(params) {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        if input.rule_set_name.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing RuleSetName");
        }
        let mut state = state.write().await;
        match state.delete_receipt_rule_set(&input.rule_set_name) {
            Ok(_) => wire::serialize_delete_receipt_rule_set_response(
                &wire::DeleteReceiptRuleSetResponse {},
            ),
            Err(e) => ses_error_from(&e),
        }
    }

    async fn handle_clone_receipt_rule_set(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_clone_receipt_rule_set_request(params) {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        if input.rule_set_name.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing RuleSetName");
        }
        if input.original_rule_set_name.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing OriginalRuleSetName");
        }
        let mut state = state.write().await;
        match state.clone_receipt_rule_set(&input.rule_set_name, &input.original_rule_set_name) {
            Ok(_) => wire::serialize_clone_receipt_rule_set_response(
                &wire::CloneReceiptRuleSetResponse {},
            ),
            Err(e) => ses_error_from(&e),
        }
    }

    async fn handle_set_active_receipt_rule_set(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_set_active_receipt_rule_set_request(params) {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        let mut state = state.write().await;
        match state.set_active_receipt_rule_set(input.rule_set_name.as_deref()) {
            Ok(_) => wire::serialize_set_active_receipt_rule_set_response(
                &wire::SetActiveReceiptRuleSetResponse {},
            ),
            Err(e) => ses_error_from(&e),
        }
    }

    async fn handle_describe_active_receipt_rule_set(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
    ) -> MockResponse {
        let state = state.read().await;
        let result = match state.describe_active_receipt_rule_set() {
            Some((name, rs)) => wire::DescribeActiveReceiptRuleSetResponse {
                metadata: Some(wire::ReceiptRuleSetMetadata {
                    name: Some(name.to_string()),
                    created_timestamp: Some(rs.created_at.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                }),
                rules: Some(wire::ReceiptRulesList {
                    items: rs.rules.iter().map(rule_to_wire).collect(),
                }),
            },
            None => wire::DescribeActiveReceiptRuleSetResponse {
                metadata: None,
                rules: None,
            },
        };
        wire::serialize_describe_active_receipt_rule_set_response(&result)
    }

    async fn handle_describe_receipt_rule_set(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_receipt_rule_set_request(params) {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        if input.rule_set_name.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing RuleSetName");
        }
        let state = state.read().await;
        match state.describe_receipt_rule_set(&input.rule_set_name) {
            Ok(rs) => wire::serialize_describe_receipt_rule_set_response(
                &wire::DescribeReceiptRuleSetResponse {
                    metadata: Some(wire::ReceiptRuleSetMetadata {
                        name: Some(rs.name.clone()),
                        created_timestamp: Some(
                            rs.created_at.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                        ),
                    }),
                    rules: Some(wire::ReceiptRulesList {
                        items: rs.rules.iter().map(rule_to_wire).collect(),
                    }),
                },
            ),
            Err(e) => ses_error_from(&e),
        }
    }

    async fn handle_list_receipt_rule_sets(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
    ) -> MockResponse {
        let state = state.read().await;
        let rule_sets = state.list_receipt_rule_sets();
        let members: Vec<wire::ReceiptRuleSetMetadata> = rule_sets
            .into_iter()
            .map(|rs| wire::ReceiptRuleSetMetadata {
                name: Some(rs.name.clone()),
                created_timestamp: Some(rs.created_at.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
            })
            .collect();
        wire::serialize_list_receipt_rule_sets_response(&wire::ListReceiptRuleSetsResponse {
            rule_sets: if members.is_empty() {
                None
            } else {
                Some(wire::ReceiptRuleSetsLists::from(members))
            },
            next_token: None,
        })
    }

    async fn handle_create_receipt_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_receipt_rule_request(params) {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        if input.rule_set_name.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing RuleSetName");
        }
        if input.rule.name.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing Rule.Name");
        }
        let rule = ReceiptRule {
            name: input.rule.name,
            enabled: input.rule.enabled.unwrap_or(false),
            scan_enabled: input.rule.scan_enabled.unwrap_or(false),
            tls_policy: input.rule.tls_policy,
        };
        let mut state = state.write().await;
        match state.create_receipt_rule(&input.rule_set_name, rule) {
            Ok(_) => {
                wire::serialize_create_receipt_rule_response(&wire::CreateReceiptRuleResponse {})
            }
            Err(e) => ses_error_from(&e),
        }
    }

    async fn handle_describe_receipt_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_receipt_rule_request(params) {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        if input.rule_set_name.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing RuleSetName");
        }
        if input.rule_name.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing RuleName");
        }
        let state = state.read().await;
        match state.describe_receipt_rule(&input.rule_set_name, &input.rule_name) {
            Ok(rule) => {
                wire::serialize_describe_receipt_rule_response(&wire::DescribeReceiptRuleResponse {
                    rule: Some(rule_to_wire(rule)),
                })
            }
            Err(e) => ses_error_from(&e),
        }
    }

    // --- Templates ---

    async fn handle_create_template(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_template_request(params) {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        if input.template.template_name.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing Template.TemplateName");
        }
        let template = Template {
            name: input.template.template_name,
            subject_part: input.template.subject_part,
            html_part: input.template.html_part,
            text_part: input.template.text_part,
            created_at: Utc::now(),
        };
        let mut state = state.write().await;
        match state.create_template(template) {
            Ok(_) => wire::serialize_create_template_response(&wire::CreateTemplateResponse {}),
            Err(e) => ses_error_from(&e),
        }
    }

    async fn handle_delete_template(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_template_request(params) {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        if input.template_name.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing TemplateName");
        }
        let mut state = state.write().await;
        match state.delete_template(&input.template_name) {
            Ok(_) => wire::serialize_delete_template_response(&wire::DeleteTemplateResponse {}),
            Err(e) => ses_error_from(&e),
        }
    }

    async fn handle_get_template(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_template_request(params) {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        if input.template_name.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing TemplateName");
        }
        let state = state.read().await;
        match state.get_template(&input.template_name) {
            Ok(t) => wire::serialize_get_template_response(&wire::GetTemplateResponse {
                template: Some(wire::Template {
                    template_name: t.name.clone(),
                    subject_part: t.subject_part.clone(),
                    html_part: t.html_part.clone(),
                    text_part: t.text_part.clone(),
                }),
            }),
            Err(e) => ses_error_from(&e),
        }
    }

    async fn handle_list_templates(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
    ) -> MockResponse {
        let state = state.read().await;
        let templates = state.list_templates();
        let members: Vec<wire::TemplateMetadata> = templates
            .into_iter()
            .map(|t| wire::TemplateMetadata {
                name: Some(t.name.clone()),
                created_timestamp: Some(t.created_at.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
            })
            .collect();
        wire::serialize_list_templates_response(&wire::ListTemplatesResponse {
            templates_metadata: if members.is_empty() {
                None
            } else {
                Some(wire::TemplateMetadataList::from(members))
            },
            next_token: None,
        })
    }

    // --- Send operations ---

    async fn handle_send_email(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_send_email_request(params) {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        if input.source.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing Source");
        }
        let to = input
            .destination
            .to_addresses
            .map(|l| l.items)
            .unwrap_or_default();
        let cc = input
            .destination
            .cc_addresses
            .map(|l| l.items)
            .unwrap_or_default();
        let bcc = input
            .destination
            .bcc_addresses
            .map(|l| l.items)
            .unwrap_or_default();
        if to.is_empty() && cc.is_empty() && bcc.is_empty() {
            return ses_error(
                400,
                "InvalidParameter",
                "At least one destination address is required",
            );
        }
        let subject = input.message.subject.data;
        let text_body = input.message.body.text.map(|c| c.data);
        let html_body = input.message.body.html.map(|c| c.data);
        let mut state = state.write().await;
        let message_id =
            state.record_sent_email(input.source, to, cc, bcc, subject, text_body, html_body);
        wire::serialize_send_email_response(&wire::SendEmailResponse {
            message_id: Some(message_id),
        })
    }

    async fn handle_send_raw_email(&self, _params: &HashMap<String, String>) -> MockResponse {
        let message_id = format!(
            "01000000000000000000000000000000-{}-000000",
            uuid::Uuid::new_v4()
        );
        wire::serialize_send_raw_email_response(&wire::SendRawEmailResponse {
            message_id: Some(message_id),
        })
    }

    async fn handle_send_templated_email(&self, _params: &HashMap<String, String>) -> MockResponse {
        let message_id = format!(
            "01000000000000000000000000000000-{}-000000",
            uuid::Uuid::new_v4()
        );
        wire::serialize_send_templated_email_response(&wire::SendTemplatedEmailResponse {
            message_id: Some(message_id),
        })
    }

    async fn handle_send_bulk_templated_email(
        &self,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        // Count Destinations.member.N entries
        let mut count = 0;
        let mut i = 1;
        loop {
            if params.contains_key(&format!(
                "Destinations.member.{i}.Destination.ToAddresses.member.1"
            )) {
                count += 1;
                i += 1;
            } else {
                break;
            }
        }
        if count == 0 {
            count = 1; // return at least one status
        }
        let statuses: Vec<wire::BulkEmailDestinationStatus> = (0..count)
            .map(|_| wire::BulkEmailDestinationStatus {
                message_id: Some(format!(
                    "01000000000000000000000000000000-{}-000000",
                    uuid::Uuid::new_v4()
                )),
                status: Some("Success".to_string()),
                error: None,
            })
            .collect();
        wire::serialize_send_bulk_templated_email_response(&wire::SendBulkTemplatedEmailResponse {
            status: Some(wire::BulkEmailDestinationStatusList { items: statuses }),
        })
    }

    async fn handle_update_template(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_template_request(params) {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        if input.template.template_name.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing Template.TemplateName");
        }
        let template = crate::types::Template {
            name: input.template.template_name,
            subject_part: input.template.subject_part,
            html_part: input.template.html_part,
            text_part: input.template.text_part,
            created_at: chrono::Utc::now(),
        };
        let mut state = state.write().await;
        match state.update_template(template) {
            Ok(_) => wire::serialize_update_template_response(&wire::UpdateTemplateResponse {}),
            Err(e) => ses_error_from(&e),
        }
    }

    async fn handle_update_receipt_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<SesV1State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_receipt_rule_request(params) {
            Ok(v) => v,
            Err(e) => return ses_error(400, "InvalidParameterValue", &e),
        };
        if input.rule_set_name.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing RuleSetName");
        }
        if input.rule.name.is_empty() {
            return ses_error(400, "InvalidParameter", "Missing Rule.Name");
        }
        let rule = crate::types::ReceiptRule {
            name: input.rule.name,
            enabled: input.rule.enabled.unwrap_or(false),
            scan_enabled: input.rule.scan_enabled.unwrap_or(false),
            tls_policy: input.rule.tls_policy,
        };
        let mut state = state.write().await;
        match state.update_receipt_rule(&input.rule_set_name, rule) {
            Ok(_) => {
                wire::serialize_update_receipt_rule_response(&wire::UpdateReceiptRuleResponse {})
            }
            Err(e) => ses_error_from(&e),
        }
    }

    async fn handle_update_configuration_set_reputation_metrics_enabled(
        &self,
        _state: &Arc<tokio::sync::RwLock<SesV1State>>,
        _params: &HashMap<String, String>,
    ) -> MockResponse {
        wire::serialize_update_configuration_set_reputation_metrics_enabled_response()
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Parse URL-encoded query string into a map (handles + for space, %xx).
fn parse_query_string(s: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for pair in s.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            let key = urldecode(key);
            let value = urldecode(value);
            map.insert(key, value);
        }
    }
    map
}

fn urldecode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.bytes();
    while let Some(b) = chars.next() {
        match b {
            b'+' => result.push(' '),
            b'%' => {
                let hi = chars.next().and_then(hex_val);
                let lo = chars.next().and_then(hex_val);
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    result.push((hi << 4 | lo) as char);
                }
            }
            _ => result.push(b as char),
        }
    }
    result
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

/// Escape characters that must not appear unescaped in XML text content.
fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Build an awsQuery response that contains a single map field.
///
/// The `entries` string should contain pre-built `<entry><key>K</key><value>V</value></entry>`
/// elements. This helper wraps them in the standard awsQuery envelope so that
/// map responses are not subject to the `quick_xml::se` limitation with
/// `HashMap<String, T>` (which cannot produce the `<entry>` format).
fn ses_map_response(op_name: &str, map_field_name: &str, entries: &str) -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<{op_name}Response xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  <{op_name}Result>
    <{map_field_name}>{entries}</{map_field_name}>
  </{op_name}Result>
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</{op_name}Response>"#
    );
    MockResponse::xml(200, xml)
}

fn ses_error(status: u16, code: &str, message: &str) -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ErrorResponse xmlns="http://ses.amazonaws.com/doc/2010-12-01/">
  <Error>
    <Type>Sender</Type>
    <Code>{code}</Code>
    <Message>{message}</Message>
  </Error>
  <RequestId>{request_id}</RequestId>
</ErrorResponse>"#
    );
    MockResponse::xml(status, xml)
}

fn ses_error_from(e: &SesV1Error) -> MockResponse {
    let (status, error_type) = match e {
        SesV1Error::NotFound { .. } => (400, "MessageRejected"),
        SesV1Error::AlreadyExists { .. } => (400, "AlreadyExists"),
    };
    ses_error(status, error_type, &e.to_string())
}

fn rule_to_wire(rule: &crate::types::ReceiptRule) -> wire::ReceiptRule {
    wire::ReceiptRule {
        name: rule.name.clone(),
        enabled: Some(rule.enabled),
        scan_enabled: Some(rule.scan_enabled),
        tls_policy: rule.tls_policy.clone(),
        ..Default::default()
    }
}
