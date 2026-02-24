use aws_sdk_sns::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_sns::SnsService;

async fn make_sns_client() -> aws_sdk_sns::Client {
    let mock = MockAws::builder().with_service(SnsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_sns::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_sns::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_list_topics() {
    let client = make_sns_client().await;

    let resp = client
        .create_topic()
        .name("test-topic")
        .send()
        .await
        .expect("create_topic should succeed");

    assert!(resp.topic_arn().is_some());
    let arn = resp.topic_arn().unwrap();
    assert!(arn.contains("test-topic"));

    let list_resp = client
        .list_topics()
        .send()
        .await
        .expect("list_topics should succeed");

    assert_eq!(list_resp.topics().len(), 1);
}

#[tokio::test]
async fn test_delete_topic() {
    let client = make_sns_client().await;

    let resp = client
        .create_topic()
        .name("del-topic")
        .send()
        .await
        .unwrap();
    let arn = resp.topic_arn().unwrap();

    client
        .delete_topic()
        .topic_arn(arn)
        .send()
        .await
        .expect("delete_topic should succeed");

    let list_resp = client.list_topics().send().await.unwrap();
    assert_eq!(list_resp.topics().len(), 0);
}

#[tokio::test]
async fn test_get_topic_attributes() {
    let client = make_sns_client().await;

    let resp = client
        .create_topic()
        .name("attr-topic")
        .send()
        .await
        .unwrap();
    let arn = resp.topic_arn().unwrap();

    let attr_resp = client
        .get_topic_attributes()
        .topic_arn(arn)
        .send()
        .await
        .expect("get_topic_attributes should succeed");

    let attrs = attr_resp.attributes().unwrap();
    assert!(attrs.contains_key("TopicArn"));
}

#[tokio::test]
async fn test_subscribe_and_list() {
    let client = make_sns_client().await;

    let resp = client
        .create_topic()
        .name("sub-topic")
        .send()
        .await
        .unwrap();
    let topic_arn = resp.topic_arn().unwrap();

    let sub_resp = client
        .subscribe()
        .topic_arn(topic_arn)
        .protocol("email")
        .endpoint("test@example.com")
        .send()
        .await
        .expect("subscribe should succeed");

    assert!(sub_resp.subscription_arn().is_some());

    let list_resp = client
        .list_subscriptions()
        .send()
        .await
        .expect("list_subscriptions should succeed");

    assert_eq!(list_resp.subscriptions().len(), 1);
}

#[tokio::test]
async fn test_unsubscribe() {
    let client = make_sns_client().await;

    let resp = client
        .create_topic()
        .name("unsub-topic")
        .send()
        .await
        .unwrap();
    let topic_arn = resp.topic_arn().unwrap();

    let sub_resp = client
        .subscribe()
        .topic_arn(topic_arn)
        .protocol("sqs")
        .endpoint("arn:aws:sqs:us-east-1:123456789012:my-queue")
        .send()
        .await
        .unwrap();
    let sub_arn = sub_resp.subscription_arn().unwrap();

    client
        .unsubscribe()
        .subscription_arn(sub_arn)
        .send()
        .await
        .expect("unsubscribe should succeed");

    let list_resp = client.list_subscriptions().send().await.unwrap();
    assert_eq!(list_resp.subscriptions().len(), 0);
}

#[tokio::test]
async fn test_publish() {
    let client = make_sns_client().await;

    let resp = client
        .create_topic()
        .name("pub-topic")
        .send()
        .await
        .unwrap();
    let topic_arn = resp.topic_arn().unwrap();

    let pub_resp = client
        .publish()
        .topic_arn(topic_arn)
        .message("Hello SNS!")
        .send()
        .await
        .expect("publish should succeed");

    assert!(pub_resp.message_id().is_some());
}

#[tokio::test]
async fn test_create_topic_idempotent() {
    let client = make_sns_client().await;

    let resp1 = client
        .create_topic()
        .name("idem-topic")
        .send()
        .await
        .unwrap();
    let resp2 = client
        .create_topic()
        .name("idem-topic")
        .send()
        .await
        .unwrap();

    assert_eq!(resp1.topic_arn(), resp2.topic_arn());
}

#[tokio::test]
async fn test_delete_nonexistent_topic_idempotent() {
    let client = make_sns_client().await;

    // SNS delete_topic is idempotent - succeeds even for nonexistent topics
    client
        .delete_topic()
        .topic_arn("arn:aws:sns:us-east-1:123456789012:no-such-topic")
        .send()
        .await
        .expect("delete_topic should succeed even for nonexistent topic");
}

#[tokio::test]
async fn test_get_nonexistent_topic_attributes_fails() {
    let client = make_sns_client().await;

    let result = client
        .get_topic_attributes()
        .topic_arn("arn:aws:sns:us-east-1:123456789012:no-such-topic")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_subscribe_to_nonexistent_topic_fails() {
    let client = make_sns_client().await;

    let result = client
        .subscribe()
        .topic_arn("arn:aws:sns:us-east-1:123456789012:no-such-topic")
        .protocol("email")
        .endpoint("test@example.com")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_publish_to_nonexistent_topic_fails() {
    let client = make_sns_client().await;

    let result = client
        .publish()
        .topic_arn("arn:aws:sns:us-east-1:123456789012:no-such-topic")
        .message("hello")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_topics_empty() {
    let client = make_sns_client().await;

    let resp = client
        .list_topics()
        .send()
        .await
        .expect("list_topics should succeed");

    assert!(resp.topics().is_empty());
}

#[tokio::test]
async fn test_list_subscriptions_empty() {
    let client = make_sns_client().await;

    let resp = client
        .list_subscriptions()
        .send()
        .await
        .expect("list_subscriptions should succeed");

    assert!(resp.subscriptions().is_empty());
}

#[tokio::test]
async fn test_unsubscribe_invalid_arn() {
    let client = make_sns_client().await;

    let result = client
        .unsubscribe()
        .subscription_arn(
            "arn:aws:sns:us-east-1:123456789012:no-topic:00000000-0000-0000-0000-000000000000",
        )
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_publish_batch() {
    let client = make_sns_client().await;

    let resp = client
        .create_topic()
        .name("batch-topic")
        .send()
        .await
        .unwrap();
    let topic_arn = resp.topic_arn().unwrap().to_string();

    let result = client
        .publish_batch()
        .topic_arn(&topic_arn)
        .publish_batch_request_entries(
            aws_sdk_sns::types::PublishBatchRequestEntry::builder()
                .id("msg-1")
                .message("Hello batch 1")
                .build()
                .unwrap(),
        )
        .publish_batch_request_entries(
            aws_sdk_sns::types::PublishBatchRequestEntry::builder()
                .id("msg-2")
                .message("Hello batch 2")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("publish_batch should succeed");

    assert_eq!(result.successful().len(), 2);
    assert!(result.failed().is_empty());
}

#[tokio::test]
async fn test_add_and_remove_permission() {
    let client = make_sns_client().await;

    let resp = client
        .create_topic()
        .name("perm-topic")
        .send()
        .await
        .unwrap();
    let topic_arn = resp.topic_arn().unwrap().to_string();

    // AddPermission should succeed
    client
        .add_permission()
        .topic_arn(&topic_arn)
        .label("AllowPublish")
        .aws_account_id("111122223333")
        .action_name("Publish")
        .send()
        .await
        .expect("add_permission should succeed");

    // RemovePermission should succeed
    client
        .remove_permission()
        .topic_arn(&topic_arn)
        .label("AllowPublish")
        .send()
        .await
        .expect("remove_permission should succeed");
}

#[tokio::test]
async fn test_get_and_set_sms_attributes() {
    let client = make_sns_client().await;

    // GetSMSAttributes on empty state should succeed
    let get_resp = client
        .get_sms_attributes()
        .send()
        .await
        .expect("get_sms_attributes should succeed");

    // Initially empty
    assert!(
        get_resp
            .attributes()
            .unwrap_or(&Default::default())
            .is_empty()
    );

    // SetSMSAttributes
    client
        .set_sms_attributes()
        .attributes("DefaultSMSType", "Transactional")
        .send()
        .await
        .expect("set_sms_attributes should succeed");

    // Get again - should have the attribute
    let get_resp2 = client
        .get_sms_attributes()
        .send()
        .await
        .expect("get_sms_attributes should succeed after set");

    let attrs = get_resp2
        .attributes()
        .expect("attributes should be present");
    assert_eq!(
        attrs.get("DefaultSMSType").map(|s| s.as_str()),
        Some("Transactional")
    );
}

// ============================================================================
// Tests derived from AWS documentation: Amazon Simple Notification Service
// ============================================================================

// Covers FIX(terraform-e2e): Added Owner, Policy, and EffectiveDeliveryPolicy attributes.
// Terraform parses Policy as JSON; missing or empty string causes parse failure.
#[tokio::test]
async fn test_get_topic_attributes_default_fields() {
    let client = make_sns_client().await;

    let resp = client
        .create_topic()
        .name("default-attrs-topic")
        .send()
        .await
        .unwrap();
    let arn = resp.topic_arn().unwrap().to_string();

    let attr_resp = client
        .get_topic_attributes()
        .topic_arn(&arn)
        .send()
        .await
        .expect("get_topic_attributes should succeed");

    let attrs = attr_resp
        .attributes()
        .expect("attributes map must be present");

    // TopicArn must be present and match
    assert_eq!(
        attrs.get("TopicArn").map(|s| s.as_str()),
        Some(arn.as_str())
    );

    // Owner must be present (account id)
    assert!(
        attrs.contains_key("Owner"),
        "Owner attribute must be present"
    );
    assert!(!attrs["Owner"].is_empty(), "Owner must not be empty");

    // Policy must be present and valid JSON
    let policy_str = attrs
        .get("Policy")
        .expect("Policy attribute must be present");
    let policy_json: serde_json::Value =
        serde_json::from_str(policy_str).expect("Policy must be valid JSON");
    assert!(
        policy_json.get("Version").is_some(),
        "Policy JSON must have a Version field"
    );

    // EffectiveDeliveryPolicy must be present and valid JSON
    let edp_str = attrs
        .get("EffectiveDeliveryPolicy")
        .expect("EffectiveDeliveryPolicy must be present");
    let _edp_json: serde_json::Value =
        serde_json::from_str(edp_str).expect("EffectiveDeliveryPolicy must be valid JSON");
}

// Covers FIX(terraform-e2e): Added SetTopicAttributes — terraform calls it after CreateTopic
// to set DisplayName and other attributes.
#[tokio::test]
async fn test_set_topic_attributes_display_name() {
    let client = make_sns_client().await;

    let resp = client
        .create_topic()
        .name("set-attr-topic")
        .send()
        .await
        .unwrap();
    let arn = resp.topic_arn().unwrap().to_string();

    // Set DisplayName via SetTopicAttributes
    client
        .set_topic_attributes()
        .topic_arn(&arn)
        .attribute_name("DisplayName")
        .attribute_value("My Display Name")
        .send()
        .await
        .expect("set_topic_attributes should succeed");

    // Verify DisplayName is reflected in GetTopicAttributes
    let attr_resp = client
        .get_topic_attributes()
        .topic_arn(&arn)
        .send()
        .await
        .unwrap();

    let attrs = attr_resp.attributes().unwrap();
    assert_eq!(
        attrs.get("DisplayName").map(|s| s.as_str()),
        Some("My Display Name"),
        "DisplayName must reflect the value set via SetTopicAttributes"
    );
}

// Covers FIX(terraform-e2e): TagResource/UntagResource/ListTagsForResource added for terraform.
#[tokio::test]
async fn test_tag_resource_lifecycle() {
    let client = make_sns_client().await;

    let resp = client
        .create_topic()
        .name("tag-lifecycle-topic")
        .send()
        .await
        .unwrap();
    let arn = resp.topic_arn().unwrap().to_string();

    // Tag the resource
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(
            aws_sdk_sns::types::Tag::builder()
                .key("env")
                .value("test")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_sns::types::Tag::builder()
                .key("team")
                .value("platform")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    // List tags — both tags must appear
    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = list_resp.tags();
    assert_eq!(tags.len(), 2, "Expected 2 tags");
    let tag_map: std::collections::HashMap<&str, &str> =
        tags.iter().map(|t| (t.key(), t.value())).collect();
    assert_eq!(tag_map.get("env"), Some(&"test"));
    assert_eq!(tag_map.get("team"), Some(&"platform"));

    // Untag one key
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("team")
        .send()
        .await
        .expect("untag_resource should succeed");

    // List tags again — only "env" should remain
    let list_resp2 = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    let tags2 = list_resp2.tags();
    assert_eq!(tags2.len(), 1, "Expected 1 tag after untag");
    assert_eq!(tags2[0].key(), "env");
    assert_eq!(tags2[0].value(), "test");
}

// Covers FIX(terraform-e2e): TagResource returns ResourceNotFoundException for nonexistent topic.
#[tokio::test]
async fn test_tag_nonexistent_resource_fails() {
    let client = make_sns_client().await;

    let result = client
        .tag_resource()
        .resource_arn("arn:aws:sns:us-east-1:123456789012:no-such-topic")
        .tags(
            aws_sdk_sns::types::Tag::builder()
                .key("key")
                .value("val")
                .build()
                .unwrap(),
        )
        .send()
        .await;

    assert!(
        result.is_err(),
        "tag_resource on nonexistent topic should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("NotFound"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// CreateTopic with tags passed at creation time — tags must be stored and listable.
#[tokio::test]
async fn test_create_topic_with_tags() {
    let client = make_sns_client().await;

    let resp = client
        .create_topic()
        .name("tagged-at-create-topic")
        .tags(
            aws_sdk_sns::types::Tag::builder()
                .key("created-by")
                .value("write-tests")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_topic with tags should succeed");

    let arn = resp.topic_arn().unwrap().to_string();

    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = list_resp.tags();
    assert_eq!(tags.len(), 1, "Expected 1 tag from creation");
    assert_eq!(tags[0].key(), "created-by");
    assert_eq!(tags[0].value(), "write-tests");
}

// DeleteTopic should also remove all subscriptions to that topic.
#[tokio::test]
async fn test_delete_topic_removes_subscriptions() {
    let client = make_sns_client().await;

    let resp = client
        .create_topic()
        .name("del-with-subs-topic")
        .send()
        .await
        .unwrap();
    let arn = resp.topic_arn().unwrap().to_string();

    // Subscribe to the topic
    client
        .subscribe()
        .topic_arn(&arn)
        .protocol("email")
        .endpoint("sub@example.com")
        .send()
        .await
        .unwrap();

    // Verify subscription is listed
    let list_before = client.list_subscriptions().send().await.unwrap();
    assert_eq!(list_before.subscriptions().len(), 1);

    // Delete the topic
    client
        .delete_topic()
        .topic_arn(&arn)
        .send()
        .await
        .expect("delete_topic should succeed");

    // Subscriptions for the deleted topic must be gone
    let list_after = client.list_subscriptions().send().await.unwrap();
    assert_eq!(
        list_after.subscriptions().len(),
        0,
        "DeleteTopic must remove subscriptions for that topic"
    );
}

// ListSubscriptions returns subscription fields: TopicArn, Protocol, Endpoint, Owner.
#[tokio::test]
async fn test_subscription_fields() {
    let client = make_sns_client().await;

    let resp = client
        .create_topic()
        .name("sub-fields-topic")
        .send()
        .await
        .unwrap();
    let topic_arn = resp.topic_arn().unwrap().to_string();

    client
        .subscribe()
        .topic_arn(&topic_arn)
        .protocol("sqs")
        .endpoint("arn:aws:sqs:us-east-1:123456789012:my-queue")
        .send()
        .await
        .unwrap();

    let list_resp = client.list_subscriptions().send().await.unwrap();
    assert_eq!(list_resp.subscriptions().len(), 1);

    let sub = &list_resp.subscriptions()[0];
    assert_eq!(sub.topic_arn(), Some(topic_arn.as_str()));
    assert_eq!(sub.protocol(), Some("sqs"));
    assert_eq!(
        sub.endpoint(),
        Some("arn:aws:sqs:us-east-1:123456789012:my-queue")
    );
    assert!(sub.owner().is_some(), "Subscription owner must be present");
}

// PublishBatch to a nonexistent topic must fail.
#[tokio::test]
async fn test_publish_batch_to_nonexistent_topic_fails() {
    let client = make_sns_client().await;

    let result = client
        .publish_batch()
        .topic_arn("arn:aws:sns:us-east-1:123456789012:no-such-topic")
        .publish_batch_request_entries(
            aws_sdk_sns::types::PublishBatchRequestEntry::builder()
                .id("msg-1")
                .message("hello")
                .build()
                .unwrap(),
        )
        .send()
        .await;

    assert!(
        result.is_err(),
        "publish_batch to nonexistent topic should fail"
    );
}

// ============================================================================
// Coverage for FIX(terraform-e2e) handler fixes
// ============================================================================

// Covers FIX(terraform-e2e): SetTopicAttributes on nonexistent topic returns error.
#[tokio::test]
async fn test_set_topic_attributes_nonexistent_fails() {
    let client = make_sns_client().await;

    let result = client
        .set_topic_attributes()
        .topic_arn("arn:aws:sns:us-east-1:123456789012:no-such-topic")
        .attribute_name("DisplayName")
        .attribute_value("Doesn't matter")
        .send()
        .await;

    assert!(
        result.is_err(),
        "set_topic_attributes on nonexistent topic should fail"
    );
}

// Full lifecycle: create → get attributes → set attribute → tag → publish → delete → verify gone.
#[tokio::test]
async fn test_full_topic_lifecycle() {
    let client = make_sns_client().await;

    // Create
    let create_resp = client
        .create_topic()
        .name("lifecycle-topic")
        .send()
        .await
        .expect("create_topic should succeed");
    let arn = create_resp.topic_arn().unwrap().to_string();
    assert!(arn.contains("lifecycle-topic"));

    // GetTopicAttributes — default fields present
    let attr_resp = client
        .get_topic_attributes()
        .topic_arn(&arn)
        .send()
        .await
        .unwrap();
    let attrs = attr_resp.attributes().unwrap();
    assert!(attrs.contains_key("TopicArn"));
    assert!(attrs.contains_key("Owner"));
    assert!(attrs.contains_key("Policy"));

    // SetTopicAttributes
    client
        .set_topic_attributes()
        .topic_arn(&arn)
        .attribute_name("DisplayName")
        .attribute_value("Lifecycle Topic")
        .send()
        .await
        .expect("set_topic_attributes should succeed");

    // TagResource
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(
            aws_sdk_sns::types::Tag::builder()
                .key("stage")
                .value("lifecycle")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    // Publish
    let pub_resp = client
        .publish()
        .topic_arn(&arn)
        .message("lifecycle message")
        .send()
        .await
        .expect("publish should succeed");
    assert!(
        !pub_resp.message_id().unwrap_or_default().is_empty(),
        "MessageId must not be empty"
    );

    // Delete
    client
        .delete_topic()
        .topic_arn(&arn)
        .send()
        .await
        .expect("delete_topic should succeed");

    // Verify gone — GetTopicAttributes must fail
    let result = client.get_topic_attributes().topic_arn(&arn).send().await;
    assert!(
        result.is_err(),
        "GetTopicAttributes on deleted topic must fail"
    );
}

// Covers FIX(terraform-e2e): ListTagsForResource on nonexistent topic returns error.
#[tokio::test]
async fn test_list_tags_for_resource_not_found() {
    let client = make_sns_client().await;

    let result = client
        .list_tags_for_resource()
        .resource_arn("arn:aws:sns:us-east-1:123456789012:no-such-topic")
        .send()
        .await;

    assert!(
        result.is_err(),
        "ListTagsForResource on nonexistent topic should fail"
    );
}

// Covers FIX(terraform-e2e): SetTopicAttributes with a custom attribute name (not just DisplayName).
#[tokio::test]
async fn test_set_topic_attributes_custom() {
    let client = make_sns_client().await;

    let resp = client
        .create_topic()
        .name("custom-attr-topic")
        .send()
        .await
        .unwrap();
    let arn = resp.topic_arn().unwrap().to_string();

    // Set a custom attribute
    client
        .set_topic_attributes()
        .topic_arn(&arn)
        .attribute_name("DeliveryPolicy")
        .attribute_value("{\"http\":{\"defaultHealthyRetryPolicy\":{\"numRetries\":5}}}")
        .send()
        .await
        .expect("set_topic_attributes with custom attribute should succeed");

    // Verify the attribute is reflected
    let attr_resp = client
        .get_topic_attributes()
        .topic_arn(&arn)
        .send()
        .await
        .unwrap();

    let attrs = attr_resp.attributes().unwrap();
    assert!(
        attrs.contains_key("DeliveryPolicy"),
        "Custom attribute DeliveryPolicy should be present after SetTopicAttributes"
    );
    assert!(
        attrs["DeliveryPolicy"].contains("numRetries"),
        "DeliveryPolicy value should match what was set"
    );
}

// Create multiple topics and verify all appear in ListTopics.
#[tokio::test]
async fn test_multiple_topics_list() {
    let client = make_sns_client().await;

    for name in &["multi-topic-1", "multi-topic-2", "multi-topic-3"] {
        client.create_topic().name(*name).send().await.unwrap();
    }

    let list_resp = client.list_topics().send().await.unwrap();
    assert_eq!(
        list_resp.topics().len(),
        3,
        "All 3 created topics should appear in ListTopics"
    );
}

// ============================================================================
// Tests for 25 new operations
// ============================================================================

// --- ConfirmSubscription ---

#[tokio::test]
async fn test_confirm_subscription() {
    let client = make_sns_client().await;

    let resp = client
        .create_topic()
        .name("confirm-sub-topic")
        .send()
        .await
        .unwrap();
    let topic_arn = resp.topic_arn().unwrap().to_string();

    let confirm_resp = client
        .confirm_subscription()
        .topic_arn(&topic_arn)
        .token("fake-token-123")
        .send()
        .await
        .expect("confirm_subscription should succeed");

    assert!(
        confirm_resp.subscription_arn().is_some(),
        "SubscriptionArn must be present"
    );
}

#[tokio::test]
async fn test_confirm_subscription_nonexistent_topic_fails() {
    let client = make_sns_client().await;

    let result = client
        .confirm_subscription()
        .topic_arn("arn:aws:sns:us-east-1:123456789012:no-such-topic")
        .token("fake-token")
        .send()
        .await;

    assert!(result.is_err());
}

// --- GetSubscriptionAttributes / SetSubscriptionAttributes ---

#[tokio::test]
async fn test_get_and_set_subscription_attributes() {
    let client = make_sns_client().await;

    let resp = client
        .create_topic()
        .name("sub-attr-topic")
        .send()
        .await
        .unwrap();
    let topic_arn = resp.topic_arn().unwrap().to_string();

    let sub_resp = client
        .subscribe()
        .topic_arn(&topic_arn)
        .protocol("sqs")
        .endpoint("arn:aws:sqs:us-east-1:123456789012:my-queue")
        .send()
        .await
        .unwrap();
    let sub_arn = sub_resp.subscription_arn().unwrap().to_string();

    // GetSubscriptionAttributes - should have default attributes
    let attr_resp = client
        .get_subscription_attributes()
        .subscription_arn(&sub_arn)
        .send()
        .await
        .expect("get_subscription_attributes should succeed");

    let attrs = attr_resp.attributes().expect("attributes must be present");
    assert_eq!(attrs.get("Protocol").map(|s| s.as_str()), Some("sqs"));
    assert_eq!(
        attrs.get("TopicArn").map(|s| s.as_str()),
        Some(topic_arn.as_str())
    );

    // SetSubscriptionAttributes - set FilterPolicy
    client
        .set_subscription_attributes()
        .subscription_arn(&sub_arn)
        .attribute_name("FilterPolicy")
        .attribute_value(r#"{"event": ["order_placed"]}"#)
        .send()
        .await
        .expect("set_subscription_attributes should succeed");

    // Verify
    let attr_resp2 = client
        .get_subscription_attributes()
        .subscription_arn(&sub_arn)
        .send()
        .await
        .unwrap();

    let attrs2 = attr_resp2.attributes().unwrap();
    assert!(
        attrs2.get("FilterPolicy").is_some(),
        "FilterPolicy attribute should be present after SetSubscriptionAttributes"
    );
}

#[tokio::test]
async fn test_get_subscription_attributes_nonexistent_fails() {
    let client = make_sns_client().await;

    let result = client
        .get_subscription_attributes()
        .subscription_arn(
            "arn:aws:sns:us-east-1:123456789012:topic:00000000-0000-0000-0000-000000000000",
        )
        .send()
        .await;

    assert!(result.is_err());
}

// --- ListSubscriptionsByTopic ---

#[tokio::test]
async fn test_list_subscriptions_by_topic() {
    let client = make_sns_client().await;

    let resp1 = client
        .create_topic()
        .name("list-by-topic-1")
        .send()
        .await
        .unwrap();
    let topic_arn1 = resp1.topic_arn().unwrap().to_string();

    let resp2 = client
        .create_topic()
        .name("list-by-topic-2")
        .send()
        .await
        .unwrap();
    let topic_arn2 = resp2.topic_arn().unwrap().to_string();

    // Subscribe to topic 1 twice, topic 2 once
    client
        .subscribe()
        .topic_arn(&topic_arn1)
        .protocol("email")
        .endpoint("a@example.com")
        .send()
        .await
        .unwrap();
    client
        .subscribe()
        .topic_arn(&topic_arn1)
        .protocol("sqs")
        .endpoint("arn:aws:sqs:us-east-1:123456789012:q1")
        .send()
        .await
        .unwrap();
    client
        .subscribe()
        .topic_arn(&topic_arn2)
        .protocol("email")
        .endpoint("b@example.com")
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_subscriptions_by_topic()
        .topic_arn(&topic_arn1)
        .send()
        .await
        .expect("list_subscriptions_by_topic should succeed");

    assert_eq!(
        list_resp.subscriptions().len(),
        2,
        "Topic 1 should have 2 subscriptions"
    );

    let list_resp2 = client
        .list_subscriptions_by_topic()
        .topic_arn(&topic_arn2)
        .send()
        .await
        .unwrap();

    assert_eq!(
        list_resp2.subscriptions().len(),
        1,
        "Topic 2 should have 1 subscription"
    );
}

// --- DataProtectionPolicy ---

#[tokio::test]
async fn test_data_protection_policy() {
    let client = make_sns_client().await;

    let resp = client
        .create_topic()
        .name("dpp-topic")
        .send()
        .await
        .unwrap();
    let topic_arn = resp.topic_arn().unwrap().to_string();

    // Initially no policy
    let get_resp = client
        .get_data_protection_policy()
        .resource_arn(&topic_arn)
        .send()
        .await
        .expect("get_data_protection_policy should succeed");

    // DataProtectionPolicy may be empty/None initially
    let _ = get_resp.data_protection_policy();

    // Put a policy
    let policy = r#"{"Name":"MyPolicy","Statement":[]}"#;
    client
        .put_data_protection_policy()
        .resource_arn(&topic_arn)
        .data_protection_policy(policy)
        .send()
        .await
        .expect("put_data_protection_policy should succeed");

    // Get the policy back
    let get_resp2 = client
        .get_data_protection_policy()
        .resource_arn(&topic_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(
        get_resp2.data_protection_policy(),
        Some(policy),
        "DataProtectionPolicy should match what was set"
    );
}

#[tokio::test]
async fn test_data_protection_policy_nonexistent_topic_fails() {
    let client = make_sns_client().await;

    let result = client
        .get_data_protection_policy()
        .resource_arn("arn:aws:sns:us-east-1:123456789012:no-such-topic")
        .send()
        .await;

    assert!(result.is_err());
}

// --- Platform Application lifecycle ---

#[tokio::test]
async fn test_platform_application_lifecycle() {
    let client = make_sns_client().await;

    // Create
    let create_resp = client
        .create_platform_application()
        .name("MyGCMApp")
        .platform("GCM")
        .attributes("PlatformCredential", "API_KEY_HERE")
        .send()
        .await
        .expect("create_platform_application should succeed");

    let app_arn = create_resp
        .platform_application_arn()
        .expect("PlatformApplicationArn must be present")
        .to_string();
    assert!(app_arn.contains("GCM"));
    assert!(app_arn.contains("MyGCMApp"));

    // GetPlatformApplicationAttributes
    let get_resp = client
        .get_platform_application_attributes()
        .platform_application_arn(&app_arn)
        .send()
        .await
        .expect("get_platform_application_attributes should succeed");

    let attrs = get_resp.attributes().expect("attributes must be present");
    assert_eq!(
        attrs.get("PlatformCredential").map(|s| s.as_str()),
        Some("API_KEY_HERE")
    );

    // SetPlatformApplicationAttributes
    client
        .set_platform_application_attributes()
        .platform_application_arn(&app_arn)
        .attributes("PlatformCredential", "NEW_KEY")
        .send()
        .await
        .expect("set_platform_application_attributes should succeed");

    // Verify update
    let get_resp2 = client
        .get_platform_application_attributes()
        .platform_application_arn(&app_arn)
        .send()
        .await
        .unwrap();
    let attrs2 = get_resp2.attributes().unwrap();
    assert_eq!(
        attrs2.get("PlatformCredential").map(|s| s.as_str()),
        Some("NEW_KEY")
    );

    // ListPlatformApplications
    let list_resp = client
        .list_platform_applications()
        .send()
        .await
        .expect("list_platform_applications should succeed");

    assert_eq!(
        list_resp.platform_applications().len(),
        1,
        "Should have 1 platform application"
    );

    // DeletePlatformApplication
    client
        .delete_platform_application()
        .platform_application_arn(&app_arn)
        .send()
        .await
        .expect("delete_platform_application should succeed");

    // Verify gone
    let list_resp2 = client.list_platform_applications().send().await.unwrap();
    assert!(
        list_resp2.platform_applications().is_empty(),
        "No platform applications after delete"
    );
}

// --- Platform Endpoint lifecycle ---

#[tokio::test]
async fn test_platform_endpoint_lifecycle() {
    let client = make_sns_client().await;

    // Create platform application first
    let app_resp = client
        .create_platform_application()
        .name("MyAPNSApp")
        .platform("APNS")
        .attributes("PlatformCredential", "cert")
        .attributes("PlatformPrincipal", "key")
        .send()
        .await
        .unwrap();
    let app_arn = app_resp.platform_application_arn().unwrap().to_string();

    // CreatePlatformEndpoint
    let ep_resp = client
        .create_platform_endpoint()
        .platform_application_arn(&app_arn)
        .token("device-token-abc123")
        .custom_user_data("user42")
        .send()
        .await
        .expect("create_platform_endpoint should succeed");

    let ep_arn = ep_resp
        .endpoint_arn()
        .expect("EndpointArn must be present")
        .to_string();

    // GetEndpointAttributes
    let get_resp = client
        .get_endpoint_attributes()
        .endpoint_arn(&ep_arn)
        .send()
        .await
        .expect("get_endpoint_attributes should succeed");

    let attrs = get_resp.attributes().expect("attributes must be present");
    assert_eq!(
        attrs.get("Token").map(|s| s.as_str()),
        Some("device-token-abc123")
    );
    assert_eq!(attrs.get("Enabled").map(|s| s.as_str()), Some("true"));
    assert_eq!(
        attrs.get("CustomUserData").map(|s| s.as_str()),
        Some("user42")
    );

    // SetEndpointAttributes
    client
        .set_endpoint_attributes()
        .endpoint_arn(&ep_arn)
        .attributes("Enabled", "false")
        .send()
        .await
        .expect("set_endpoint_attributes should succeed");

    // Verify update
    let get_resp2 = client
        .get_endpoint_attributes()
        .endpoint_arn(&ep_arn)
        .send()
        .await
        .unwrap();
    let attrs2 = get_resp2.attributes().unwrap();
    assert_eq!(attrs2.get("Enabled").map(|s| s.as_str()), Some("false"));

    // ListEndpointsByPlatformApplication
    let list_resp = client
        .list_endpoints_by_platform_application()
        .platform_application_arn(&app_arn)
        .send()
        .await
        .expect("list_endpoints_by_platform_application should succeed");

    assert_eq!(list_resp.endpoints().len(), 1, "Should have 1 endpoint");

    // DeleteEndpoint
    client
        .delete_endpoint()
        .endpoint_arn(&ep_arn)
        .send()
        .await
        .expect("delete_endpoint should succeed");

    // Verify gone
    let list_resp2 = client
        .list_endpoints_by_platform_application()
        .platform_application_arn(&app_arn)
        .send()
        .await
        .unwrap();
    assert!(list_resp2.endpoints().is_empty());
}

#[tokio::test]
async fn test_create_platform_endpoint_nonexistent_app_fails() {
    let client = make_sns_client().await;

    let result = client
        .create_platform_endpoint()
        .platform_application_arn("arn:aws:sns:us-east-1:123456789012:app/GCM/NoSuchApp")
        .token("some-token")
        .send()
        .await;

    assert!(result.is_err());
}

// --- SMS Sandbox lifecycle ---

#[tokio::test]
async fn test_sms_sandbox_lifecycle() {
    let client = make_sns_client().await;

    // CreateSMSSandboxPhoneNumber
    client
        .create_sms_sandbox_phone_number()
        .phone_number("+12065551234")
        .send()
        .await
        .expect("create_sms_sandbox_phone_number should succeed");

    // ListSMSSandboxPhoneNumbers
    let list_resp = client
        .list_sms_sandbox_phone_numbers()
        .send()
        .await
        .expect("list_sms_sandbox_phone_numbers should succeed");

    let numbers = list_resp.phone_numbers();
    assert_eq!(numbers.len(), 1);
    assert_eq!(numbers[0].phone_number(), Some("+12065551234"));
    assert_eq!(numbers[0].status().map(|s| s.as_str()), Some("Pending"));

    // VerifySMSSandboxPhoneNumber
    client
        .verify_sms_sandbox_phone_number()
        .phone_number("+12065551234")
        .one_time_password("123456")
        .send()
        .await
        .expect("verify_sms_sandbox_phone_number should succeed");

    // Verify status changed to Verified
    let list_resp2 = client
        .list_sms_sandbox_phone_numbers()
        .send()
        .await
        .unwrap();
    let numbers2 = list_resp2.phone_numbers();
    assert_eq!(numbers2[0].status().map(|s| s.as_str()), Some("Verified"));

    // DeleteSMSSandboxPhoneNumber
    client
        .delete_sms_sandbox_phone_number()
        .phone_number("+12065551234")
        .send()
        .await
        .expect("delete_sms_sandbox_phone_number should succeed");

    // Verify gone
    let list_resp3 = client
        .list_sms_sandbox_phone_numbers()
        .send()
        .await
        .unwrap();
    assert!(list_resp3.phone_numbers().is_empty());
}

#[tokio::test]
async fn test_delete_sms_sandbox_phone_number_not_found() {
    let client = make_sns_client().await;

    let result = client
        .delete_sms_sandbox_phone_number()
        .phone_number("+19995551111")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_verify_sms_sandbox_phone_number_not_found() {
    let client = make_sns_client().await;

    let result = client
        .verify_sms_sandbox_phone_number()
        .phone_number("+19995551111")
        .one_time_password("000000")
        .send()
        .await;

    assert!(result.is_err());
}

// --- GetSMSSandboxAccountStatus ---

#[tokio::test]
async fn test_get_sms_sandbox_account_status() {
    let client = make_sns_client().await;

    let resp = client
        .get_sms_sandbox_account_status()
        .send()
        .await
        .expect("get_sms_sandbox_account_status should succeed");

    // Default is false (not in sandbox) for our mock
    let _is_in_sandbox = resp.is_in_sandbox();
    // Just verify the call succeeds
}

// --- SMS Opt-out operations ---

#[tokio::test]
async fn test_check_if_phone_number_is_opted_out() {
    let client = make_sns_client().await;

    let resp = client
        .check_if_phone_number_is_opted_out()
        .phone_number("+12065559999")
        .send()
        .await
        .expect("check_if_phone_number_is_opted_out should succeed");

    assert!(
        !resp.is_opted_out(),
        "Phone number should not be opted out by default"
    );
}

#[tokio::test]
async fn test_list_phone_numbers_opted_out() {
    let client = make_sns_client().await;

    let resp = client
        .list_phone_numbers_opted_out()
        .send()
        .await
        .expect("list_phone_numbers_opted_out should succeed");

    assert!(
        resp.phone_numbers().is_empty(),
        "No phone numbers should be opted out by default"
    );
}

#[tokio::test]
async fn test_opt_in_phone_number() {
    let client = make_sns_client().await;

    // OptInPhoneNumber should succeed even if not opted out
    client
        .opt_in_phone_number()
        .phone_number("+12065559999")
        .send()
        .await
        .expect("opt_in_phone_number should succeed");
}

// --- ListOriginationNumbers ---

#[tokio::test]
async fn test_list_origination_numbers() {
    let client = make_sns_client().await;

    let resp = client
        .list_origination_numbers()
        .send()
        .await
        .expect("list_origination_numbers should succeed");

    // Empty list expected for mock
    assert!(resp.phone_numbers().is_empty());
}

// --- Delete platform application removes endpoints ---

#[tokio::test]
async fn test_delete_platform_application_removes_endpoints() {
    let client = make_sns_client().await;

    let app_resp = client
        .create_platform_application()
        .name("DeleteApp")
        .platform("GCM")
        .attributes("PlatformCredential", "key")
        .send()
        .await
        .unwrap();
    let app_arn = app_resp.platform_application_arn().unwrap().to_string();

    // Create an endpoint
    let ep_resp = client
        .create_platform_endpoint()
        .platform_application_arn(&app_arn)
        .token("token1")
        .send()
        .await
        .unwrap();
    let ep_arn = ep_resp.endpoint_arn().unwrap().to_string();

    // Delete the app
    client
        .delete_platform_application()
        .platform_application_arn(&app_arn)
        .send()
        .await
        .unwrap();

    // Endpoint should be gone
    let result = client
        .get_endpoint_attributes()
        .endpoint_arn(&ep_arn)
        .send()
        .await;
    assert!(
        result.is_err(),
        "Endpoint should be deleted with platform app"
    );
}

// --- SetSubscriptionAttributes on nonexistent subscription ---

#[tokio::test]
async fn test_set_subscription_attributes_nonexistent_fails() {
    let client = make_sns_client().await;

    let result = client
        .set_subscription_attributes()
        .subscription_arn(
            "arn:aws:sns:us-east-1:123456789012:topic:00000000-0000-0000-0000-000000000000",
        )
        .attribute_name("FilterPolicy")
        .attribute_value("{}")
        .send()
        .await;

    assert!(result.is_err());
}

// --- ListSubscriptionsByTopic on nonexistent topic ---

#[tokio::test]
async fn test_list_subscriptions_by_topic_nonexistent_fails() {
    let client = make_sns_client().await;

    let result = client
        .list_subscriptions_by_topic()
        .topic_arn("arn:aws:sns:us-east-1:123456789012:no-such-topic")
        .send()
        .await;

    assert!(result.is_err());
}

// --- Multiple platform applications ---

#[tokio::test]
async fn test_list_platform_applications_multiple() {
    let client = make_sns_client().await;

    client
        .create_platform_application()
        .name("App1")
        .platform("GCM")
        .attributes("PlatformCredential", "key1")
        .send()
        .await
        .unwrap();

    client
        .create_platform_application()
        .name("App2")
        .platform("APNS")
        .attributes("PlatformCredential", "cert1")
        .attributes("PlatformPrincipal", "key1")
        .send()
        .await
        .unwrap();

    let list_resp = client.list_platform_applications().send().await.unwrap();

    assert_eq!(
        list_resp.platform_applications().len(),
        2,
        "Should have 2 platform applications"
    );
}

// --- GetPlatformApplicationAttributes nonexistent ---

#[tokio::test]
async fn test_get_platform_application_attributes_nonexistent_fails() {
    let client = make_sns_client().await;

    let result = client
        .get_platform_application_attributes()
        .platform_application_arn("arn:aws:sns:us-east-1:123456789012:app/GCM/NoSuchApp")
        .send()
        .await;

    assert!(result.is_err());
}

// --- GetEndpointAttributes nonexistent ---

#[tokio::test]
async fn test_get_endpoint_attributes_nonexistent_fails() {
    let client = make_sns_client().await;

    let result = client
        .get_endpoint_attributes()
        .endpoint_arn("arn:aws:sns:us-east-1:123456789012:endpoint/GCM/NoSuchApp/00000000-0000-0000-0000-000000000000")
        .send()
        .await;

    assert!(result.is_err());
}
