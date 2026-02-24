//! Integration tests for winterbaume-sesv1 using the real aws-sdk-ses client.

use aws_sdk_ses::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_ses::SesService;

async fn make_client() -> aws_sdk_ses::Client {
    let mock = MockAws::builder().with_service(SesService::new()).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ses::config::Region::new("us-east-1"))
        .load()
        .await;
    // SES v1 uses email.*.amazonaws.com by default, but our mock service handles
    // ses.*.amazonaws.com to avoid conflicts with winterbaume-ses (SES v2) in server deployments.
    aws_sdk_ses::Client::from_conf(
        aws_sdk_ses::config::Builder::from(&config)
            .endpoint_url("https://ses.us-east-1.amazonaws.com")
            .build(),
    )
}

// ---------------------------------------------------------------------------
// Identity tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_verify_email_address() {
    let client = make_client().await;
    client
        .verify_email_address()
        .email_address("test@example.com")
        .send()
        .await
        .expect("VerifyEmailAddress should succeed");
}

#[tokio::test]
async fn test_verify_email_identity() {
    let client = make_client().await;
    client
        .verify_email_identity()
        .email_address("test2@example.com")
        .send()
        .await
        .expect("VerifyEmailIdentity should succeed");
}

#[tokio::test]
async fn test_verify_domain_identity() {
    let client = make_client().await;
    let resp = client
        .verify_domain_identity()
        .domain("example.com")
        .send()
        .await
        .expect("VerifyDomainIdentity should succeed");
    assert!(
        !resp.verification_token().is_empty(),
        "verification token should be non-empty"
    );
}

#[tokio::test]
async fn test_list_identities_empty() {
    let client = make_client().await;
    let resp = client
        .list_identities()
        .send()
        .await
        .expect("ListIdentities should succeed");
    assert!(resp.identities().is_empty(), "should start empty");
}

#[tokio::test]
async fn test_list_identities_after_verify() {
    let client = make_client().await;
    client
        .verify_email_address()
        .email_address("test@example.com")
        .send()
        .await
        .unwrap();
    let resp = client.list_identities().send().await.unwrap();
    assert!(resp.identities().contains(&"test@example.com".to_string()));
}

#[tokio::test]
async fn test_delete_identity() {
    let client = make_client().await;
    client
        .verify_email_address()
        .email_address("delete@example.com")
        .send()
        .await
        .unwrap();
    client
        .delete_identity()
        .identity("delete@example.com")
        .send()
        .await
        .expect("DeleteIdentity should succeed");
    let resp = client.list_identities().send().await.unwrap();
    assert!(
        !resp
            .identities()
            .contains(&"delete@example.com".to_string())
    );
}

#[tokio::test]
async fn test_list_verified_email_addresses() {
    let client = make_client().await;
    client
        .verify_email_address()
        .email_address("verified@example.com")
        .send()
        .await
        .unwrap();
    let resp = client
        .list_verified_email_addresses()
        .send()
        .await
        .expect("ListVerifiedEmailAddresses should succeed");
    assert!(
        resp.verified_email_addresses()
            .contains(&"verified@example.com".to_string())
    );
}

#[tokio::test]
async fn test_get_identity_verification_attributes() {
    let client = make_client().await;
    client
        .verify_email_address()
        .email_address("attr@example.com")
        .send()
        .await
        .unwrap();
    let resp = client
        .get_identity_verification_attributes()
        .identities("attr@example.com")
        .send()
        .await
        .expect("GetIdentityVerificationAttributes should succeed");
    let attrs = resp.verification_attributes();
    assert!(attrs.contains_key("attr@example.com"));
    let attr = attrs.get("attr@example.com").unwrap();
    assert_eq!(
        attr.verification_status(),
        &aws_sdk_ses::types::VerificationStatus::Success
    );
}

#[tokio::test]
async fn test_get_identity_dkim_attributes() {
    let client = make_client().await;
    client
        .verify_domain_identity()
        .domain("dkim.example.com")
        .send()
        .await
        .unwrap();
    let resp = client
        .get_identity_dkim_attributes()
        .identities("dkim.example.com")
        .send()
        .await
        .expect("GetIdentityDkimAttributes should succeed");
    let attrs = resp.dkim_attributes();
    assert!(attrs.contains_key("dkim.example.com"));
}

#[tokio::test]
async fn test_get_identity_mail_from_domain_attributes() {
    let client = make_client().await;
    client
        .verify_email_address()
        .email_address("mailfrom@example.com")
        .send()
        .await
        .unwrap();
    let resp = client
        .get_identity_mail_from_domain_attributes()
        .identities("mailfrom@example.com")
        .send()
        .await
        .expect("GetIdentityMailFromDomainAttributes should succeed");
    assert!(
        resp.mail_from_domain_attributes()
            .contains_key("mailfrom@example.com")
    );
}

#[tokio::test]
async fn test_get_identity_notification_attributes() {
    let client = make_client().await;
    client
        .verify_email_address()
        .email_address("notif@example.com")
        .send()
        .await
        .unwrap();
    let resp = client
        .get_identity_notification_attributes()
        .identities("notif@example.com")
        .send()
        .await
        .expect("GetIdentityNotificationAttributes should succeed");
    assert!(
        resp.notification_attributes()
            .contains_key("notif@example.com")
    );
}

#[tokio::test]
async fn test_set_identity_feedback_forwarding_enabled() {
    let client = make_client().await;
    client
        .verify_email_address()
        .email_address("fwd@example.com")
        .send()
        .await
        .unwrap();
    client
        .set_identity_feedback_forwarding_enabled()
        .identity("fwd@example.com")
        .forwarding_enabled(false)
        .send()
        .await
        .expect("SetIdentityFeedbackForwardingEnabled should succeed");
}

#[tokio::test]
async fn test_set_identity_mail_from_domain() {
    let client = make_client().await;
    client
        .verify_email_address()
        .email_address("mfrom@example.com")
        .send()
        .await
        .unwrap();
    client
        .set_identity_mail_from_domain()
        .identity("mfrom@example.com")
        .mail_from_domain("bounce.example.com")
        .send()
        .await
        .expect("SetIdentityMailFromDomain should succeed");
}

#[tokio::test]
async fn test_set_identity_notification_topic() {
    let client = make_client().await;
    client
        .verify_email_address()
        .email_address("ntopic@example.com")
        .send()
        .await
        .unwrap();
    client
        .set_identity_notification_topic()
        .identity("ntopic@example.com")
        .notification_type(aws_sdk_ses::types::NotificationType::Bounce)
        .sns_topic("arn:aws:sns:us-east-1:123456789012:bounce-topic")
        .send()
        .await
        .expect("SetIdentityNotificationTopic should succeed");
}

// ---------------------------------------------------------------------------
// Send quota / statistics
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_get_send_quota() {
    let client = make_client().await;
    let resp = client
        .get_send_quota()
        .send()
        .await
        .expect("GetSendQuota should succeed");
    assert!(resp.max24_hour_send() > 0.0);
    assert!(resp.max_send_rate() > 0.0);
}

#[tokio::test]
async fn test_get_send_statistics() {
    let client = make_client().await;
    client
        .get_send_statistics()
        .send()
        .await
        .expect("GetSendStatistics should succeed");
}

// ---------------------------------------------------------------------------
// SendEmail
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_send_email() {
    let client = make_client().await;
    let resp = client
        .send_email()
        .source("sender@example.com")
        .destination(
            aws_sdk_ses::types::Destination::builder()
                .to_addresses("recipient@example.com")
                .build(),
        )
        .message(
            aws_sdk_ses::types::Message::builder()
                .subject(
                    aws_sdk_ses::types::Content::builder()
                        .data("Test Subject")
                        .build()
                        .unwrap(),
                )
                .body(
                    aws_sdk_ses::types::Body::builder()
                        .text(
                            aws_sdk_ses::types::Content::builder()
                                .data("Test Body")
                                .build()
                                .unwrap(),
                        )
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .expect("SendEmail should succeed");
    assert!(!resp.message_id().is_empty());
}

#[tokio::test]
async fn test_send_raw_email() {
    let client = make_client().await;
    let raw_msg =
        "From: sender@example.com\r\nTo: recipient@example.com\r\nSubject: Test\r\n\r\nBody";
    let resp = client
        .send_raw_email()
        .raw_message(
            aws_sdk_ses::types::RawMessage::builder()
                .data(aws_smithy_types::Blob::new(raw_msg.as_bytes()))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("SendRawEmail should succeed");
    assert!(!resp.message_id().is_empty());
}

// ---------------------------------------------------------------------------
// Configuration Sets
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_create_and_list_configuration_sets() {
    let client = make_client().await;
    client
        .create_configuration_set()
        .configuration_set(
            aws_sdk_ses::types::ConfigurationSet::builder()
                .name("my-config-set")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("CreateConfigurationSet should succeed");

    let resp = client
        .list_configuration_sets()
        .send()
        .await
        .expect("ListConfigurationSets should succeed");
    let names: Vec<&str> = resp
        .configuration_sets()
        .iter()
        .map(|cs| cs.name())
        .collect();
    assert!(names.contains(&"my-config-set"));
}

#[tokio::test]
async fn test_describe_and_delete_configuration_set() {
    let client = make_client().await;
    client
        .create_configuration_set()
        .configuration_set(
            aws_sdk_ses::types::ConfigurationSet::builder()
                .name("to-delete-cs")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_configuration_set()
        .configuration_set_name("to-delete-cs")
        .send()
        .await
        .expect("DescribeConfigurationSet should succeed");
    assert_eq!(resp.configuration_set().unwrap().name(), "to-delete-cs");

    client
        .delete_configuration_set()
        .configuration_set_name("to-delete-cs")
        .send()
        .await
        .expect("DeleteConfigurationSet should succeed");

    // After deletion, listing should not contain it
    let resp = client.list_configuration_sets().send().await.unwrap();
    let names: Vec<&str> = resp
        .configuration_sets()
        .iter()
        .map(|cs| cs.name())
        .collect();
    assert!(!names.contains(&"to-delete-cs"));
}

// ---------------------------------------------------------------------------
// Receipt Rule Sets
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_create_and_list_receipt_rule_sets() {
    let client = make_client().await;
    client
        .create_receipt_rule_set()
        .rule_set_name("my-rule-set")
        .send()
        .await
        .expect("CreateReceiptRuleSet should succeed");

    let resp = client
        .list_receipt_rule_sets()
        .send()
        .await
        .expect("ListReceiptRuleSets should succeed");
    let names: Vec<Option<&str>> = resp.rule_sets().iter().map(|rs| rs.name()).collect();
    assert!(names.contains(&Some("my-rule-set")));
}

#[tokio::test]
async fn test_describe_receipt_rule_set() {
    let client = make_client().await;
    client
        .create_receipt_rule_set()
        .rule_set_name("describe-test-set")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_receipt_rule_set()
        .rule_set_name("describe-test-set")
        .send()
        .await
        .expect("DescribeReceiptRuleSet should succeed");
    assert_eq!(resp.metadata().unwrap().name(), Some("describe-test-set"));
}

#[tokio::test]
async fn test_clone_receipt_rule_set() {
    let client = make_client().await;
    client
        .create_receipt_rule_set()
        .rule_set_name("original-set")
        .send()
        .await
        .unwrap();
    client
        .clone_receipt_rule_set()
        .rule_set_name("cloned-set")
        .original_rule_set_name("original-set")
        .send()
        .await
        .expect("CloneReceiptRuleSet should succeed");

    let resp = client.list_receipt_rule_sets().send().await.unwrap();
    let names: Vec<Option<&str>> = resp.rule_sets().iter().map(|rs| rs.name()).collect();
    assert!(names.contains(&Some("cloned-set")));
}

#[tokio::test]
async fn test_set_and_describe_active_receipt_rule_set() {
    let client = make_client().await;
    client
        .create_receipt_rule_set()
        .rule_set_name("active-set")
        .send()
        .await
        .unwrap();
    client
        .set_active_receipt_rule_set()
        .rule_set_name("active-set")
        .send()
        .await
        .expect("SetActiveReceiptRuleSet should succeed");

    let resp = client
        .describe_active_receipt_rule_set()
        .send()
        .await
        .expect("DescribeActiveReceiptRuleSet should succeed");
    assert_eq!(resp.metadata().unwrap().name(), Some("active-set"));
}

#[tokio::test]
async fn test_delete_receipt_rule_set() {
    let client = make_client().await;
    client
        .create_receipt_rule_set()
        .rule_set_name("delete-set")
        .send()
        .await
        .unwrap();
    client
        .delete_receipt_rule_set()
        .rule_set_name("delete-set")
        .send()
        .await
        .expect("DeleteReceiptRuleSet should succeed");
    let resp = client.list_receipt_rule_sets().send().await.unwrap();
    let names: Vec<Option<&str>> = resp.rule_sets().iter().map(|rs| rs.name()).collect();
    assert!(!names.contains(&Some("delete-set")));
}

#[tokio::test]
async fn test_create_and_describe_receipt_rule() {
    let client = make_client().await;
    client
        .create_receipt_rule_set()
        .rule_set_name("rule-test-set")
        .send()
        .await
        .unwrap();
    client
        .create_receipt_rule()
        .rule_set_name("rule-test-set")
        .rule(
            aws_sdk_ses::types::ReceiptRule::builder()
                .name("my-rule")
                .enabled(true)
                .scan_enabled(true)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("CreateReceiptRule should succeed");

    let resp = client
        .describe_receipt_rule()
        .rule_set_name("rule-test-set")
        .rule_name("my-rule")
        .send()
        .await
        .expect("DescribeReceiptRule should succeed");
    assert_eq!(resp.rule().unwrap().name(), "my-rule");
}

// ---------------------------------------------------------------------------
// Templates
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_create_get_list_delete_template() {
    let client = make_client().await;
    client
        .create_template()
        .template(
            aws_sdk_ses::types::Template::builder()
                .template_name("my-template")
                .subject_part("Hello {{name}}")
                .html_part("<p>Hello {{name}}</p>")
                .text_part("Hello {{name}}")
                .build()
                .expect("Template build should succeed"),
        )
        .send()
        .await
        .expect("CreateTemplate should succeed");

    let resp = client
        .get_template()
        .template_name("my-template")
        .send()
        .await
        .expect("GetTemplate should succeed");
    assert_eq!(resp.template().unwrap().template_name(), "my-template");

    let resp = client
        .list_templates()
        .send()
        .await
        .expect("ListTemplates should succeed");
    let names: Vec<Option<&str>> = resp.templates_metadata().iter().map(|t| t.name()).collect();
    assert!(names.contains(&Some("my-template")));

    client
        .delete_template()
        .template_name("my-template")
        .send()
        .await
        .expect("DeleteTemplate should succeed");

    // After deletion, get should fail
    let err = client
        .get_template()
        .template_name("my-template")
        .send()
        .await;
    assert!(err.is_err(), "GetTemplate after delete should fail");
}

// ---------------------------------------------------------------------------
// State view tests (snapshot / restore / merge)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_state_snapshot_restore() {
    use std::collections::HashMap;

    use winterbaume_core::StatefulService;
    use winterbaume_ses::{SesV1StateView, views::IdentityView};

    let svc = SesService::new();

    // Restore a view with one identity
    let mut identities = HashMap::new();
    identities.insert(
        "snap@example.com".to_string(),
        IdentityView {
            name: "snap@example.com".to_string(),
            identity_type: "EmailAddress".to_string(),
            verification_status: "Success".to_string(),
            verification_token: None,
            dkim_tokens: vec![],
            dkim_enabled: false,
            mail_from_domain: None,
            bounce_topic: None,
            complaint_topic: None,
            delivery_topic: None,
            forwarding_enabled: true,
        },
    );
    let view = SesV1StateView {
        identities,
        ..Default::default()
    };
    svc.restore("123456789012", "us-east-1", view)
        .await
        .expect("restore should succeed");

    let snapshot = svc.snapshot("123456789012", "us-east-1").await;
    assert!(
        snapshot.identities.contains_key("snap@example.com"),
        "identity should be present after restore"
    );

    // Restore empty view, check state is cleared
    svc.restore("123456789012", "us-east-1", SesV1StateView::default())
        .await
        .expect("restore empty should succeed");
    let snapshot2 = svc.snapshot("123456789012", "us-east-1").await;
    assert!(
        snapshot2.identities.is_empty(),
        "state should be empty after restoring empty view"
    );
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;

    let svc = SesService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });

    svc.restore(
        "123456789012",
        "us-east-1",
        winterbaume_ses::SesV1StateView::default(),
    )
    .await
    .unwrap();

    let got = events.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].0, "123456789012");
    assert_eq!(got[0].1, "us-east-1");
}

#[tokio::test]
async fn test_state_merge_is_additive() {
    use std::collections::HashMap;

    use winterbaume_core::StatefulService;
    use winterbaume_ses::{SesV1StateView, views::IdentityView};

    let svc = SesService::new();

    // Restore initial state with one identity
    let mut identities1 = HashMap::new();
    identities1.insert(
        "a@example.com".to_string(),
        IdentityView {
            name: "a@example.com".to_string(),
            identity_type: "EmailAddress".to_string(),
            verification_status: "Success".to_string(),
            verification_token: None,
            dkim_tokens: vec![],
            dkim_enabled: false,
            mail_from_domain: None,
            bounce_topic: None,
            complaint_topic: None,
            delivery_topic: None,
            forwarding_enabled: true,
        },
    );
    let view1 = SesV1StateView {
        identities: identities1,
        ..Default::default()
    };
    svc.restore("123456789012", "us-east-1", view1)
        .await
        .unwrap();

    // Merge a second identity
    let mut identities2 = HashMap::new();
    identities2.insert(
        "b@example.com".to_string(),
        IdentityView {
            name: "b@example.com".to_string(),
            identity_type: "EmailAddress".to_string(),
            verification_status: "Success".to_string(),
            verification_token: None,
            dkim_tokens: vec![],
            dkim_enabled: false,
            mail_from_domain: None,
            bounce_topic: None,
            complaint_topic: None,
            delivery_topic: None,
            forwarding_enabled: true,
        },
    );
    let view2 = SesV1StateView {
        identities: identities2,
        ..Default::default()
    };
    svc.merge("123456789012", "us-east-1", view2).await.unwrap();

    // Both identities should be present
    let snapshot = svc.snapshot("123456789012", "us-east-1").await;
    assert!(
        snapshot.identities.contains_key("a@example.com"),
        "a should still be present"
    );
    assert!(
        snapshot.identities.contains_key("b@example.com"),
        "b should be present after merge"
    );
}
