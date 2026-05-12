use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::{
    MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::backend::{InMemorySnsBackend, SnsBackend};
use crate::state::SnsError;
use crate::views::SnsStateView;
use crate::wire;

pub struct SnsService {
    /// Pluggable storage backend for all topic, subscription, and state operations.
    pub(crate) backend: Arc<dyn SnsBackend>,
    pub(crate) notifier: StateChangeNotifier<SnsStateView>,
}

impl SnsService {
    pub fn new() -> Self {
        Self {
            backend: Arc::new(InMemorySnsBackend::new()),
            notifier: StateChangeNotifier::new(),
        }
    }

    /// Returns sorted `(account_id, region)` pairs that have state.
    pub fn scopes_with_state(&self) -> Vec<(String, String)> {
        self.backend.scopes_with_state()
    }

    /// Construct with a custom backend.
    pub fn with_backend(backend: Arc<dyn SnsBackend>) -> Self {
        Self {
            backend,
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for SnsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for SnsService {
    fn service_name(&self) -> &str {
        "sns"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://sns\..*\.amazonaws\.com",
            r"https?://sns\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl SnsService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id().to_string();

        let body_str = std::str::from_utf8(&request.body).unwrap_or("");
        let params = parse_query_string(body_str);

        let action = match params.get("Action") {
            Some(a) => a.as_str(),
            None => {
                return MockResponse::error(400, "MissingAction", "Missing 'Action' parameter");
            }
        };

        let response = match action {
            "CreateTopic" => {
                self.handle_create_topic(&params, account_id.clone(), region.clone())
                    .await
            }
            "DeleteTopic" => {
                self.handle_delete_topic(&params, account_id.clone(), region.clone())
                    .await
            }
            "ListTopics" => {
                self.handle_list_topics(account_id.clone(), region.clone())
                    .await
            }
            "GetTopicAttributes" => {
                self.handle_get_topic_attributes(&params, account_id.clone(), region.clone())
                    .await
            }
            "Subscribe" => {
                self.handle_subscribe(&params, account_id.clone(), region.clone())
                    .await
            }
            "Unsubscribe" => {
                self.handle_unsubscribe(&params, account_id.clone(), region.clone())
                    .await
            }
            "ListSubscriptions" => {
                self.handle_list_subscriptions(account_id.clone(), region.clone())
                    .await
            }
            "Publish" => {
                self.handle_publish(&params, account_id.clone(), region.clone())
                    .await
            }
            "PublishBatch" => {
                self.handle_publish_batch(&params, account_id.clone(), region.clone())
                    .await
            }
            "AddPermission" => {
                self.handle_add_permission(&params, account_id.clone(), region.clone())
                    .await
            }
            "RemovePermission" => {
                self.handle_remove_permission(&params, account_id.clone(), region.clone())
                    .await
            }
            "SetTopicAttributes" => {
                self.handle_set_topic_attributes(&params, account_id.clone(), region.clone())
                    .await
            }
            "TagResource" => {
                self.handle_tag_resource(&params, account_id.clone(), region.clone())
                    .await
            }
            "UntagResource" => {
                self.handle_untag_resource(&params, account_id.clone(), region.clone())
                    .await
            }
            "ListTagsForResource" => {
                self.handle_list_tags_for_resource(&params, account_id.clone(), region.clone())
                    .await
            }
            "GetSMSAttributes" => {
                self.handle_get_sms_attributes(account_id.clone(), region.clone())
                    .await
            }
            "SetSMSAttributes" => {
                self.handle_set_sms_attributes(&params, account_id.clone(), region.clone())
                    .await
            }
            "ConfirmSubscription" => {
                self.handle_confirm_subscription(&params, account_id.clone(), region.clone())
                    .await
            }
            "GetSubscriptionAttributes" => {
                self.handle_get_subscription_attributes(&params, account_id.clone(), region.clone())
                    .await
            }
            "SetSubscriptionAttributes" => {
                self.handle_set_subscription_attributes(&params, account_id.clone(), region.clone())
                    .await
            }
            "ListSubscriptionsByTopic" => {
                self.handle_list_subscriptions_by_topic(&params, account_id.clone(), region.clone())
                    .await
            }
            "GetDataProtectionPolicy" => {
                self.handle_get_data_protection_policy(&params, account_id.clone(), region.clone())
                    .await
            }
            "PutDataProtectionPolicy" => {
                self.handle_put_data_protection_policy(&params, account_id.clone(), region.clone())
                    .await
            }
            "CreatePlatformApplication" => {
                self.handle_create_platform_application(&params, account_id.clone(), region.clone())
                    .await
            }
            "DeletePlatformApplication" => {
                self.handle_delete_platform_application(&params, account_id.clone(), region.clone())
                    .await
            }
            "GetPlatformApplicationAttributes" => {
                self.handle_get_platform_application_attributes(
                    &params,
                    account_id.clone(),
                    region.clone(),
                )
                .await
            }
            "SetPlatformApplicationAttributes" => {
                self.handle_set_platform_application_attributes(
                    &params,
                    account_id.clone(),
                    region.clone(),
                )
                .await
            }
            "ListPlatformApplications" => {
                self.handle_list_platform_applications(account_id.clone(), region.clone())
                    .await
            }
            "CreatePlatformEndpoint" => {
                self.handle_create_platform_endpoint(&params, account_id.clone(), region.clone())
                    .await
            }
            "DeleteEndpoint" => {
                self.handle_delete_endpoint(&params, account_id.clone(), region.clone())
                    .await
            }
            "GetEndpointAttributes" => {
                self.handle_get_endpoint_attributes(&params, account_id.clone(), region.clone())
                    .await
            }
            "SetEndpointAttributes" => {
                self.handle_set_endpoint_attributes(&params, account_id.clone(), region.clone())
                    .await
            }
            "ListEndpointsByPlatformApplication" => {
                self.handle_list_endpoints_by_platform_application(
                    &params,
                    account_id.clone(),
                    region.clone(),
                )
                .await
            }
            "CreateSMSSandboxPhoneNumber" => {
                self.handle_create_sms_sandbox_phone_number(
                    &params,
                    account_id.clone(),
                    region.clone(),
                )
                .await
            }
            "DeleteSMSSandboxPhoneNumber" => {
                self.handle_delete_sms_sandbox_phone_number(
                    &params,
                    account_id.clone(),
                    region.clone(),
                )
                .await
            }
            "VerifySMSSandboxPhoneNumber" => {
                self.handle_verify_sms_sandbox_phone_number(
                    &params,
                    account_id.clone(),
                    region.clone(),
                )
                .await
            }
            "ListSMSSandboxPhoneNumbers" => {
                self.handle_list_sms_sandbox_phone_numbers(account_id.clone(), region.clone())
                    .await
            }
            "GetSMSSandboxAccountStatus" => {
                self.handle_get_sms_sandbox_account_status(account_id.clone(), region.clone())
                    .await
            }
            "CheckIfPhoneNumberIsOptedOut" => {
                self.handle_check_if_phone_number_is_opted_out(
                    &params,
                    account_id.clone(),
                    region.clone(),
                )
                .await
            }
            "ListPhoneNumbersOptedOut" => {
                self.handle_list_phone_numbers_opted_out(account_id.clone(), region.clone())
                    .await
            }
            "OptInPhoneNumber" => {
                self.handle_opt_in_phone_number(&params, account_id.clone(), region.clone())
                    .await
            }
            "ListOriginationNumbers" => self.handle_list_origination_numbers().await,
            _ => MockResponse::error(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for SNS"),
            ),
        };
        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 {
            self.notify_state_changed(&account_id, &region).await;
        }
        response
    }

    // ========================================================================
    // Handlers — all delegate to self.backend
    // ========================================================================

    async fn handle_create_topic(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_create_topic_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'Name'");
        }

        let tags = wire_tags_to_domain(input.tags);

        match self
            .backend
            .create_topic(account_id, region, input.name, tags)
            .await
        {
            Ok(arn) => {
                let xml = format!(
                    r#"<CreateTopicResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <CreateTopicResult>
    <TopicArn>{}</TopicArn>
  </CreateTopicResult>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</CreateTopicResponse>"#,
                    xml_escape(&arn),
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_delete_topic(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_topic_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.topic_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'TopicArn'");
        }

        match self
            .backend
            .delete_topic(account_id, region, input.topic_arn)
            .await
        {
            Ok(()) => {
                let xml = format!(
                    r#"<DeleteTopicResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</DeleteTopicResponse>"#,
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_list_topics(&self, account_id: String, region: String) -> MockResponse {
        match self.backend.list_topics(account_id, region).await {
            Ok(topics) => {
                let members: String = topics
                    .iter()
                    .map(|t| {
                        format!(
                            "    <member>\n      <TopicArn>{}</TopicArn>\n    </member>",
                            xml_escape(&t.arn)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                let xml = format!(
                    r#"<ListTopicsResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <ListTopicsResult>
    <Topics>
{members}
    </Topics>
  </ListTopicsResult>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</ListTopicsResponse>"#,
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_get_topic_attributes(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_get_topic_attributes_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.topic_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'TopicArn'");
        }

        match self
            .backend
            .get_topic_attributes(account_id, region, input.topic_arn)
            .await
        {
            Ok(attrs) => {
                let entries: String = attrs
                    .iter()
                    .map(|(k, v)| {
                        format!(
                            "      <entry>\n        <key>{}</key>\n        <value>{}</value>\n      </entry>",
                            xml_escape(k),
                            xml_escape(v)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                let xml = format!(
                    r#"<GetTopicAttributesResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <GetTopicAttributesResult>
    <Attributes>
{entries}
    </Attributes>
  </GetTopicAttributesResult>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</GetTopicAttributesResponse>"#,
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_subscribe(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_subscribe_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.topic_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'TopicArn'");
        }
        if input.protocol.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'Protocol'");
        }
        let endpoint = input.endpoint.unwrap_or_default();

        match self
            .backend
            .subscribe(
                account_id,
                region,
                input.topic_arn,
                input.protocol,
                endpoint,
            )
            .await
        {
            Ok(sub_arn) => {
                let xml = format!(
                    r#"<SubscribeResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <SubscribeResult>
    <SubscriptionArn>{}</SubscriptionArn>
  </SubscribeResult>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</SubscribeResponse>"#,
                    xml_escape(&sub_arn),
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_unsubscribe(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_unsubscribe_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.subscription_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SubscriptionArn'");
        }

        match self
            .backend
            .unsubscribe(account_id, region, input.subscription_arn)
            .await
        {
            Ok(()) => {
                let xml = format!(
                    r#"<UnsubscribeResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</UnsubscribeResponse>"#,
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_list_subscriptions(&self, account_id: String, region: String) -> MockResponse {
        match self.backend.list_subscriptions(account_id, region).await {
            Ok(subs) => {
                let members: String = subs
                    .iter()
                    .map(|s| {
                        format!(
                            "    <member>\n      <SubscriptionArn>{}</SubscriptionArn>\n      <TopicArn>{}</TopicArn>\n      <Protocol>{}</Protocol>\n      <Endpoint>{}</Endpoint>\n      <Owner>{}</Owner>\n    </member>",
                            xml_escape(&s.arn),
                            xml_escape(&s.topic_arn),
                            xml_escape(&s.protocol),
                            xml_escape(&s.endpoint),
                            xml_escape(&s.owner),
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                let xml = format!(
                    r#"<ListSubscriptionsResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <ListSubscriptionsResult>
    <Subscriptions>
{members}
    </Subscriptions>
  </ListSubscriptionsResult>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</ListSubscriptionsResponse>"#,
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_publish(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_publish_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let topic_arn = match input.topic_arn {
            Some(a) if !a.is_empty() => a,
            _ => return MockResponse::error(400, "MissingParameter", "Missing 'TopicArn'"),
        };
        if input.message.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'Message'");
        }

        match self
            .backend
            .publish(account_id, region, topic_arn, input.message)
            .await
        {
            Ok(message_id) => {
                let xml = format!(
                    r#"<PublishResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <PublishResult>
    <MessageId>{message_id}</MessageId>
  </PublishResult>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</PublishResponse>"#,
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_publish_batch(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_publish_batch_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.topic_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'TopicArn'");
        }

        let entries: Vec<(String, String)> = input
            .publish_batch_request_entries
            .items
            .into_iter()
            .map(|e| (e.id, e.message))
            .collect();

        match self
            .backend
            .publish_batch(account_id, region, input.topic_arn, entries)
            .await
        {
            Ok(results) => {
                let successful: String = results
                    .iter()
                    .map(|(batch_id, msg_id)| {
                        format!(
                            "      <member>\n        <Id>{}</Id>\n        <MessageId>{}</MessageId>\n      </member>",
                            xml_escape(batch_id),
                            xml_escape(msg_id)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                let xml = format!(
                    r#"<PublishBatchResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <PublishBatchResult>
    <Successful>
{successful}
    </Successful>
    <Failed/>
  </PublishBatchResult>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</PublishBatchResponse>"#,
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_add_permission(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_add_permission_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.topic_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'TopicArn'");
        }
        if input.label.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'Label'");
        }

        match self
            .backend
            .add_permission(
                account_id,
                region,
                input.topic_arn,
                input.label,
                input.a_w_s_account_id.items,
                input.action_name.items,
            )
            .await
        {
            Ok(()) => wire::serialize_add_permission_response(),
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_remove_permission(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_remove_permission_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.topic_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'TopicArn'");
        }
        if input.label.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'Label'");
        }

        match self
            .backend
            .remove_permission(account_id, region, input.topic_arn, input.label)
            .await
        {
            Ok(()) => wire::serialize_remove_permission_response(),
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_get_sms_attributes(&self, account_id: String, region: String) -> MockResponse {
        match self.backend.get_sms_attributes(account_id, region).await {
            Ok(attrs) => {
                let entries: String = attrs
                    .iter()
                    .map(|(k, v)| {
                        format!(
                            "      <entry>\n        <key>{}</key>\n        <value>{}</value>\n      </entry>",
                            xml_escape(k),
                            xml_escape(v)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                let xml = format!(
                    r#"<GetSMSAttributesResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <GetSMSAttributesResult>
    <attributes>
{entries}
    </attributes>
  </GetSMSAttributesResult>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</GetSMSAttributesResponse>"#,
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_set_sms_attributes(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        // The wire deserializer does not parse the attributes map; extract manually.
        let attrs = extract_entry_map(params, "attributes");

        match self
            .backend
            .set_sms_attributes(account_id, region, attrs)
            .await
        {
            Ok(()) => {
                let xml = format!(
                    r#"<SetSMSAttributesResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</SetSMSAttributesResponse>"#,
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_set_topic_attributes(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_set_topic_attributes_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.topic_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'TopicArn'");
        }
        if input.attribute_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'AttributeName'");
        }
        let attr_value = input.attribute_value.unwrap_or_default();

        match self
            .backend
            .set_topic_attributes(
                account_id,
                region,
                input.topic_arn,
                input.attribute_name,
                attr_value,
            )
            .await
        {
            Ok(()) => wire::serialize_set_topic_attributes_response(),
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.resource_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ResourceArn'");
        }

        let tags = input
            .tags
            .items
            .into_iter()
            .map(|t| (t.key, t.value))
            .collect::<HashMap<String, String>>();

        match self
            .backend
            .tag_resource(account_id, region, input.resource_arn, tags)
            .await
        {
            Ok(()) => {
                let xml = format!(
                    r#"<TagResourceResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</TagResourceResponse>"#,
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.resource_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ResourceArn'");
        }

        match self
            .backend
            .untag_resource(account_id, region, input.resource_arn, input.tag_keys.items)
            .await
        {
            Ok(()) => {
                let xml = format!(
                    r#"<UntagResourceResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</UntagResourceResponse>"#,
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.resource_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ResourceArn'");
        }

        match self
            .backend
            .list_tags_for_resource(account_id, region, input.resource_arn)
            .await
        {
            Ok(tags) => {
                let tags_xml: String = tags
                    .iter()
                    .map(|(k, v)| {
                        format!(
                            "      <member>\n        <Key>{}</Key>\n        <Value>{}</Value>\n      </member>",
                            xml_escape(k),
                            xml_escape(v)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                let xml = format!(
                    r#"<ListTagsForResourceResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <ListTagsForResourceResult>
    <Tags>
{tags_xml}
    </Tags>
  </ListTagsForResourceResult>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</ListTagsForResourceResponse>"#,
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_confirm_subscription(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_confirm_subscription_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.topic_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'TopicArn'");
        }
        if input.token.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'Token'");
        }

        match self
            .backend
            .confirm_subscription(account_id, region, input.topic_arn, input.token)
            .await
        {
            Ok(sub_arn) => {
                let xml = format!(
                    r#"<ConfirmSubscriptionResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <ConfirmSubscriptionResult>
    <SubscriptionArn>{}</SubscriptionArn>
  </ConfirmSubscriptionResult>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</ConfirmSubscriptionResponse>"#,
                    xml_escape(&sub_arn),
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_get_subscription_attributes(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_get_subscription_attributes_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.subscription_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SubscriptionArn'");
        }

        match self
            .backend
            .get_subscription_attributes(account_id, region, input.subscription_arn)
            .await
        {
            Ok(attrs) => {
                let entries: String = attrs
                    .iter()
                    .map(|(k, v)| {
                        format!(
                            "      <entry>\n        <key>{}</key>\n        <value>{}</value>\n      </entry>",
                            xml_escape(k),
                            xml_escape(v)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                let xml = format!(
                    r#"<GetSubscriptionAttributesResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <GetSubscriptionAttributesResult>
    <Attributes>
{entries}
    </Attributes>
  </GetSubscriptionAttributesResult>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</GetSubscriptionAttributesResponse>"#,
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_set_subscription_attributes(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_set_subscription_attributes_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.subscription_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SubscriptionArn'");
        }
        if input.attribute_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'AttributeName'");
        }
        let attr_value = input.attribute_value.unwrap_or_default();

        match self
            .backend
            .set_subscription_attributes(
                account_id,
                region,
                input.subscription_arn,
                input.attribute_name,
                attr_value,
            )
            .await
        {
            Ok(()) => wire::serialize_set_subscription_attributes_response(),
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_list_subscriptions_by_topic(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_list_subscriptions_by_topic_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.topic_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'TopicArn'");
        }

        match self
            .backend
            .list_subscriptions_by_topic(account_id, region, input.topic_arn)
            .await
        {
            Ok(subs) => {
                let members: String = subs
                    .iter()
                    .map(|s| {
                        format!(
                            "    <member>\n      <SubscriptionArn>{}</SubscriptionArn>\n      <TopicArn>{}</TopicArn>\n      <Protocol>{}</Protocol>\n      <Endpoint>{}</Endpoint>\n      <Owner>{}</Owner>\n    </member>",
                            xml_escape(&s.arn),
                            xml_escape(&s.topic_arn),
                            xml_escape(&s.protocol),
                            xml_escape(&s.endpoint),
                            xml_escape(&s.owner),
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                let xml = format!(
                    r#"<ListSubscriptionsByTopicResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <ListSubscriptionsByTopicResult>
    <Subscriptions>
{members}
    </Subscriptions>
  </ListSubscriptionsByTopicResult>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</ListSubscriptionsByTopicResponse>"#,
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_get_data_protection_policy(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_get_data_protection_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.resource_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ResourceArn'");
        }

        match self
            .backend
            .get_data_protection_policy(account_id, region, input.resource_arn)
            .await
        {
            Ok(policy) => {
                let policy_xml = match policy {
                    Some(p) => format!(
                        "    <DataProtectionPolicy>{}</DataProtectionPolicy>",
                        xml_escape(&p)
                    ),
                    None => String::new(),
                };
                let xml = format!(
                    r#"<GetDataProtectionPolicyResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <GetDataProtectionPolicyResult>
{policy_xml}
  </GetDataProtectionPolicyResult>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</GetDataProtectionPolicyResponse>"#,
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_put_data_protection_policy(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_put_data_protection_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.resource_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ResourceArn'");
        }
        if input.data_protection_policy.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'DataProtectionPolicy'");
        }

        match self
            .backend
            .put_data_protection_policy(
                account_id,
                region,
                input.resource_arn,
                input.data_protection_policy,
            )
            .await
        {
            Ok(()) => wire::serialize_put_data_protection_policy_response(),
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_create_platform_application(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_create_platform_application_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'Name'");
        }
        if input.platform.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'Platform'");
        }

        // The wire deserializer does not parse the Attributes map; extract manually.
        let attributes = extract_entry_map(params, "Attributes");

        match self
            .backend
            .create_platform_application(account_id, region, input.name, input.platform, attributes)
            .await
        {
            Ok(arn) => {
                let xml = format!(
                    r#"<CreatePlatformApplicationResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <CreatePlatformApplicationResult>
    <PlatformApplicationArn>{}</PlatformApplicationArn>
  </CreatePlatformApplicationResult>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</CreatePlatformApplicationResponse>"#,
                    xml_escape(&arn),
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_delete_platform_application(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_platform_application_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.platform_application_arn.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'PlatformApplicationArn'",
            );
        }

        match self
            .backend
            .delete_platform_application(account_id, region, input.platform_application_arn)
            .await
        {
            Ok(()) => wire::serialize_delete_platform_application_response(),
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_get_platform_application_attributes(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_get_platform_application_attributes_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.platform_application_arn.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'PlatformApplicationArn'",
            );
        }

        match self
            .backend
            .get_platform_application_attributes(account_id, region, input.platform_application_arn)
            .await
        {
            Ok(app) => {
                let entries = format_map_entries_xml(&app.attributes);
                let xml = format!(
                    r#"<GetPlatformApplicationAttributesResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <GetPlatformApplicationAttributesResult>
    <Attributes>
{entries}
    </Attributes>
  </GetPlatformApplicationAttributesResult>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</GetPlatformApplicationAttributesResponse>"#,
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_set_platform_application_attributes(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_set_platform_application_attributes_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.platform_application_arn.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'PlatformApplicationArn'",
            );
        }

        // The wire deserializer does not parse the Attributes map; extract manually.
        let attributes = extract_entry_map(params, "Attributes");

        match self
            .backend
            .set_platform_application_attributes(
                account_id,
                region,
                input.platform_application_arn,
                attributes,
            )
            .await
        {
            Ok(()) => wire::serialize_set_platform_application_attributes_response(),
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_list_platform_applications(
        &self,
        account_id: String,
        region: String,
    ) -> MockResponse {
        match self
            .backend
            .list_platform_applications(account_id, region)
            .await
        {
            Ok(apps) => {
                let members: String = apps
                    .iter()
                    .map(|a| {
                        let attr_entries = format_map_entries_xml(&a.attributes);
                        format!(
                            "    <member>\n      <PlatformApplicationArn>{}</PlatformApplicationArn>\n      <Attributes>\n{attr_entries}\n      </Attributes>\n    </member>",
                            xml_escape(&a.arn)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                let xml = format!(
                    r#"<ListPlatformApplicationsResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <ListPlatformApplicationsResult>
    <PlatformApplications>
{members}
    </PlatformApplications>
  </ListPlatformApplicationsResult>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</ListPlatformApplicationsResponse>"#,
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_create_platform_endpoint(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_create_platform_endpoint_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.platform_application_arn.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'PlatformApplicationArn'",
            );
        }
        if input.token.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'Token'");
        }
        // The wire deserializer does not parse the Attributes map; extract manually.
        let attributes = extract_entry_map(params, "Attributes");

        match self
            .backend
            .create_platform_endpoint(
                account_id,
                region,
                input.platform_application_arn,
                input.token,
                input.custom_user_data,
                attributes,
            )
            .await
        {
            Ok(arn) => {
                let xml = format!(
                    r#"<CreatePlatformEndpointResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <CreatePlatformEndpointResult>
    <EndpointArn>{}</EndpointArn>
  </CreatePlatformEndpointResult>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</CreatePlatformEndpointResponse>"#,
                    xml_escape(&arn),
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_delete_endpoint(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_endpoint_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.endpoint_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'EndpointArn'");
        }

        match self
            .backend
            .delete_endpoint(account_id, region, input.endpoint_arn)
            .await
        {
            Ok(()) => wire::serialize_delete_endpoint_response(),
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_get_endpoint_attributes(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_get_endpoint_attributes_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.endpoint_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'EndpointArn'");
        }

        match self
            .backend
            .get_endpoint_attributes(account_id, region, input.endpoint_arn)
            .await
        {
            Ok(endpoint) => {
                let entries = format_map_entries_xml(&endpoint.attributes);
                let xml = format!(
                    r#"<GetEndpointAttributesResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <GetEndpointAttributesResult>
    <Attributes>
{entries}
    </Attributes>
  </GetEndpointAttributesResult>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</GetEndpointAttributesResponse>"#,
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_set_endpoint_attributes(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_set_endpoint_attributes_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.endpoint_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'EndpointArn'");
        }

        // The wire deserializer does not parse the Attributes map; extract manually.
        let attributes = extract_entry_map(params, "Attributes");

        match self
            .backend
            .set_endpoint_attributes(account_id, region, input.endpoint_arn, attributes)
            .await
        {
            Ok(()) => wire::serialize_set_endpoint_attributes_response(),
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_list_endpoints_by_platform_application(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_list_endpoints_by_platform_application_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.platform_application_arn.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'PlatformApplicationArn'",
            );
        }

        match self
            .backend
            .list_endpoints_by_platform_application(
                account_id,
                region,
                input.platform_application_arn,
            )
            .await
        {
            Ok(endpoints) => {
                let members: String = endpoints
                    .iter()
                    .map(|e| {
                        let attr_entries = format_map_entries_xml(&e.attributes);
                        format!(
                            "    <member>\n      <EndpointArn>{}</EndpointArn>\n      <Attributes>\n{attr_entries}\n      </Attributes>\n    </member>",
                            xml_escape(&e.arn)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                let xml = format!(
                    r#"<ListEndpointsByPlatformApplicationResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <ListEndpointsByPlatformApplicationResult>
    <Endpoints>
{members}
    </Endpoints>
  </ListEndpointsByPlatformApplicationResult>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</ListEndpointsByPlatformApplicationResponse>"#,
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_create_sms_sandbox_phone_number(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_create_s_m_s_sandbox_phone_number_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.phone_number.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PhoneNumber'");
        }

        match self
            .backend
            .create_sms_sandbox_phone_number(account_id, region, input.phone_number)
            .await
        {
            Ok(()) => {
                let xml = format!(
                    r#"<CreateSMSSandboxPhoneNumberResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <CreateSMSSandboxPhoneNumberResult/>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</CreateSMSSandboxPhoneNumberResponse>"#,
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_delete_sms_sandbox_phone_number(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_s_m_s_sandbox_phone_number_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.phone_number.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PhoneNumber'");
        }

        match self
            .backend
            .delete_sms_sandbox_phone_number(account_id, region, input.phone_number)
            .await
        {
            Ok(()) => {
                let xml = format!(
                    r#"<DeleteSMSSandboxPhoneNumberResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <DeleteSMSSandboxPhoneNumberResult/>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</DeleteSMSSandboxPhoneNumberResponse>"#,
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_verify_sms_sandbox_phone_number(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_verify_s_m_s_sandbox_phone_number_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.phone_number.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PhoneNumber'");
        }
        if input.one_time_password.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'OneTimePassword'");
        }

        match self
            .backend
            .verify_sms_sandbox_phone_number(
                account_id,
                region,
                input.phone_number,
                input.one_time_password,
            )
            .await
        {
            Ok(()) => {
                let xml = format!(
                    r#"<VerifySMSSandboxPhoneNumberResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <VerifySMSSandboxPhoneNumberResult/>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</VerifySMSSandboxPhoneNumberResponse>"#,
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_list_sms_sandbox_phone_numbers(
        &self,
        account_id: String,
        region: String,
    ) -> MockResponse {
        match self
            .backend
            .list_sms_sandbox_phone_numbers(account_id, region)
            .await
        {
            Ok(numbers) => {
                let members: String = numbers
                    .iter()
                    .map(|p| {
                        format!(
                            "    <member>\n      <PhoneNumber>{}</PhoneNumber>\n      <Status>{}</Status>\n    </member>",
                            xml_escape(&p.phone_number),
                            xml_escape(&p.status)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                let xml = format!(
                    r#"<ListSMSSandboxPhoneNumbersResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <ListSMSSandboxPhoneNumbersResult>
    <PhoneNumbers>
{members}
    </PhoneNumbers>
  </ListSMSSandboxPhoneNumbersResult>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</ListSMSSandboxPhoneNumbersResponse>"#,
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_get_sms_sandbox_account_status(
        &self,
        account_id: String,
        region: String,
    ) -> MockResponse {
        match self
            .backend
            .get_sms_sandbox_account_status(account_id, region)
            .await
        {
            Ok(is_in_sandbox) => {
                let xml = format!(
                    r#"<GetSMSSandboxAccountStatusResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <GetSMSSandboxAccountStatusResult>
    <IsInSandbox>{}</IsInSandbox>
  </GetSMSSandboxAccountStatusResult>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</GetSMSSandboxAccountStatusResponse>"#,
                    is_in_sandbox,
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_check_if_phone_number_is_opted_out(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_check_if_phone_number_is_opted_out_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.phone_number.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'phoneNumber'");
        }

        match self
            .backend
            .check_if_phone_number_is_opted_out(account_id, region, input.phone_number)
            .await
        {
            Ok(is_opted_out) => {
                let xml = format!(
                    r#"<CheckIfPhoneNumberIsOptedOutResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <CheckIfPhoneNumberIsOptedOutResult>
    <isOptedOut>{}</isOptedOut>
  </CheckIfPhoneNumberIsOptedOutResult>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</CheckIfPhoneNumberIsOptedOutResponse>"#,
                    is_opted_out,
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_list_phone_numbers_opted_out(
        &self,
        account_id: String,
        region: String,
    ) -> MockResponse {
        match self
            .backend
            .list_phone_numbers_opted_out(account_id, region)
            .await
        {
            Ok(numbers) => {
                let members: String = numbers
                    .iter()
                    .map(|p| format!("    <member>{}</member>", xml_escape(p)))
                    .collect::<Vec<_>>()
                    .join("\n");

                let xml = format!(
                    r#"<ListPhoneNumbersOptedOutResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <ListPhoneNumbersOptedOutResult>
    <phoneNumbers>
{members}
    </phoneNumbers>
  </ListPhoneNumbersOptedOutResult>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</ListPhoneNumbersOptedOutResponse>"#,
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    async fn handle_opt_in_phone_number(
        &self,
        params: &HashMap<String, String>,
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_opt_in_phone_number_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.phone_number.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'phoneNumber'");
        }

        match self
            .backend
            .opt_in_phone_number(account_id, region, input.phone_number)
            .await
        {
            Ok(()) => {
                let xml = format!(
                    r#"<OptInPhoneNumberResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <OptInPhoneNumberResult/>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</OptInPhoneNumberResponse>"#,
                    uuid::Uuid::new_v4()
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => sns_error_response(&e),
        }
    }

    // STUB[no-state]: ListOriginationNumbers requires real phone number provisioning state;
    //   always returns an empty list.
    async fn handle_list_origination_numbers(&self) -> MockResponse {
        let xml = format!(
            r#"<ListOriginationNumbersResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
  <ListOriginationNumbersResult>
    <PhoneNumbers/>
  </ListOriginationNumbersResult>
  <ResponseMetadata>
    <RequestId>{}</RequestId>
  </ResponseMetadata>
</ListOriginationNumbersResponse>"#,
            uuid::Uuid::new_v4()
        );
        MockResponse::xml(200, xml)
    }
}

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

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Convert wire tag list to the domain HashMap<String, String>.
fn wire_tags_to_domain(tags: Option<wire::TagList>) -> HashMap<String, String> {
    tags.map(|tl| tl.items.into_iter().map(|t| (t.key, t.value)).collect())
        .unwrap_or_default()
}

/// Extract a map from awsQuery form-encoded entries like `prefix.entry.N.key` / `.value`.
fn extract_entry_map(params: &HashMap<String, String>, prefix: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for i in 1.. {
        let key_param = format!("{prefix}.entry.{i}.key");
        let val_param = format!("{prefix}.entry.{i}.value");
        match (params.get(&key_param), params.get(&val_param)) {
            (Some(k), Some(v)) => {
                map.insert(k.clone(), v.clone());
            }
            _ => break,
        }
    }
    map
}

/// Format a HashMap as XML entry elements for awsQuery map serialization.
fn format_map_entries_xml(map: &HashMap<String, String>) -> String {
    map.iter()
        .map(|(k, v)| {
            format!(
                "      <entry>\n        <key>{}</key>\n        <value>{}</value>\n      </entry>",
                xml_escape(k),
                xml_escape(v)
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn sns_error_response(err: &SnsError) -> MockResponse {
    let (status, error_type) = match err {
        SnsError::TopicNotFound => (404, "NotFound"),
        SnsError::SubscriptionNotFound => (404, "NotFound"),
        SnsError::PlatformApplicationNotFound => (404, "NotFound"),
        SnsError::EndpointNotFound => (404, "NotFound"),
        SnsError::PhoneNumberAlreadyInSandbox => (400, "OptedOutException"),
        SnsError::SandboxPhoneNumberNotFound => (404, "ResourceNotFoundException"),
        SnsError::ResourceNotFound(_) => (404, "ResourceNotFoundException"),
    };
    MockResponse::error(status, error_type, &err.to_string())
}
