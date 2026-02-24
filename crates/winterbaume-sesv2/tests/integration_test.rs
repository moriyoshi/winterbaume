use aws_sdk_sesv2::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_sesv2::SesV2Service;

async fn make_ses_client() -> aws_sdk_sesv2::Client {
    let mock = MockAws::builder().with_service(SesV2Service::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_sesv2::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_sesv2::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_get_email_identity() {
    let client = make_ses_client().await;

    let resp = client
        .create_email_identity()
        .email_identity("test@example.com")
        .send()
        .await
        .expect("create_email_identity should succeed");

    assert!(resp.verified_for_sending_status());

    let get_resp = client
        .get_email_identity()
        .email_identity("test@example.com")
        .send()
        .await
        .expect("get_email_identity should succeed");

    assert!(get_resp.verified_for_sending_status());
}

#[tokio::test]
async fn test_create_domain_identity() {
    let client = make_ses_client().await;

    let resp = client
        .create_email_identity()
        .email_identity("example.com")
        .send()
        .await
        .expect("create domain identity should succeed");

    assert!(resp.verified_for_sending_status());
}

#[tokio::test]
async fn test_delete_email_identity() {
    let client = make_ses_client().await;

    client
        .create_email_identity()
        .email_identity("del@example.com")
        .send()
        .await
        .unwrap();

    client
        .delete_email_identity()
        .email_identity("del@example.com")
        .send()
        .await
        .expect("delete should succeed");

    let result = client
        .get_email_identity()
        .email_identity("del@example.com")
        .send()
        .await;
    assert!(result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_list_email_identities() {
    let client = make_ses_client().await;

    client
        .create_email_identity()
        .email_identity("a@example.com")
        .send()
        .await
        .unwrap();

    client
        .create_email_identity()
        .email_identity("b@example.com")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_email_identities()
        .send()
        .await
        .expect("list_email_identities should succeed");

    assert_eq!(resp.email_identities().len(), 2);
}

#[tokio::test]
async fn test_send_email() {
    let client = make_ses_client().await;

    client
        .create_email_identity()
        .email_identity("sender@example.com")
        .send()
        .await
        .unwrap();

    let destination = aws_sdk_sesv2::types::Destination::builder()
        .to_addresses("recipient@example.com")
        .build();

    let subject = aws_sdk_sesv2::types::Content::builder()
        .data("Test Subject")
        .build()
        .unwrap();

    let body_text = aws_sdk_sesv2::types::Content::builder()
        .data("Hello, World!")
        .build()
        .unwrap();

    let body = aws_sdk_sesv2::types::Body::builder()
        .text(body_text)
        .build();

    let message = aws_sdk_sesv2::types::Message::builder()
        .subject(subject)
        .body(body)
        .build();

    let content = aws_sdk_sesv2::types::EmailContent::builder()
        .simple(message)
        .build();

    let resp = client
        .send_email()
        .from_email_address("sender@example.com")
        .destination(destination)
        .content(content)
        .send()
        .await
        .expect("send_email should succeed");

    assert!(resp.message_id().is_some());
}

#[tokio::test]
async fn test_create_duplicate_identity_fails() {
    let client = make_ses_client().await;

    client
        .create_email_identity()
        .email_identity("dup@example.com")
        .send()
        .await
        .unwrap();

    let result = client
        .create_email_identity()
        .email_identity("dup@example.com")
        .send()
        .await;
    assert!(result.is_err(), "duplicate identity should fail");
}

#[tokio::test]
async fn test_get_nonexistent_identity_fails() {
    let client = make_ses_client().await;

    let result = client
        .get_email_identity()
        .email_identity("nonexistent@example.com")
        .send()
        .await;
    assert!(result.is_err(), "get nonexistent identity should fail");
}

// --- Configuration Set tests ---

#[tokio::test]
async fn test_create_and_get_configuration_set() {
    let client = make_ses_client().await;

    client
        .create_configuration_set()
        .configuration_set_name("my-config-set")
        .send()
        .await
        .expect("create_configuration_set should succeed");

    let resp = client
        .get_configuration_set()
        .configuration_set_name("my-config-set")
        .send()
        .await
        .expect("get_configuration_set should succeed");

    assert_eq!(
        resp.configuration_set_name().unwrap_or_default(),
        "my-config-set"
    );
}

#[tokio::test]
async fn test_delete_configuration_set() {
    let client = make_ses_client().await;

    client
        .create_configuration_set()
        .configuration_set_name("to-delete")
        .send()
        .await
        .unwrap();

    client
        .delete_configuration_set()
        .configuration_set_name("to-delete")
        .send()
        .await
        .expect("delete_configuration_set should succeed");

    let result = client
        .get_configuration_set()
        .configuration_set_name("to-delete")
        .send()
        .await;
    assert!(result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_list_configuration_sets() {
    let client = make_ses_client().await;

    client
        .create_configuration_set()
        .configuration_set_name("cs-one")
        .send()
        .await
        .unwrap();

    client
        .create_configuration_set()
        .configuration_set_name("cs-two")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_configuration_sets()
        .send()
        .await
        .expect("list_configuration_sets should succeed");

    assert_eq!(resp.configuration_sets().len(), 2);
}

// --- Contact List tests ---

#[tokio::test]
async fn test_create_and_get_contact_list() {
    let client = make_ses_client().await;

    client
        .create_contact_list()
        .contact_list_name("my-list")
        .send()
        .await
        .expect("create_contact_list should succeed");

    let resp = client
        .get_contact_list()
        .contact_list_name("my-list")
        .send()
        .await
        .expect("get_contact_list should succeed");

    assert_eq!(resp.contact_list_name().unwrap_or_default(), "my-list");
}

#[tokio::test]
async fn test_delete_contact_list() {
    let client = make_ses_client().await;

    client
        .create_contact_list()
        .contact_list_name("list-to-delete")
        .send()
        .await
        .unwrap();

    client
        .delete_contact_list()
        .contact_list_name("list-to-delete")
        .send()
        .await
        .expect("delete_contact_list should succeed");

    let result = client
        .get_contact_list()
        .contact_list_name("list-to-delete")
        .send()
        .await;
    assert!(result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_list_contact_lists() {
    let client = make_ses_client().await;

    client
        .create_contact_list()
        .contact_list_name("list-a")
        .send()
        .await
        .unwrap();

    client
        .create_contact_list()
        .contact_list_name("list-b")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_contact_lists()
        .send()
        .await
        .expect("list_contact_lists should succeed");

    assert_eq!(resp.contact_lists().len(), 2);
}

// --- Contact tests ---

#[tokio::test]
async fn test_create_and_get_contact() {
    let client = make_ses_client().await;

    client
        .create_contact_list()
        .contact_list_name("contact-list-1")
        .send()
        .await
        .unwrap();

    client
        .create_contact()
        .contact_list_name("contact-list-1")
        .email_address("user@example.com")
        .send()
        .await
        .expect("create_contact should succeed");

    let resp = client
        .get_contact()
        .contact_list_name("contact-list-1")
        .email_address("user@example.com")
        .send()
        .await
        .expect("get_contact should succeed");

    assert_eq!(resp.email_address().unwrap_or_default(), "user@example.com");
}

#[tokio::test]
async fn test_delete_contact() {
    let client = make_ses_client().await;

    client
        .create_contact_list()
        .contact_list_name("contact-list-2")
        .send()
        .await
        .unwrap();

    client
        .create_contact()
        .contact_list_name("contact-list-2")
        .email_address("del-user@example.com")
        .send()
        .await
        .unwrap();

    client
        .delete_contact()
        .contact_list_name("contact-list-2")
        .email_address("del-user@example.com")
        .send()
        .await
        .expect("delete_contact should succeed");

    let result = client
        .get_contact()
        .contact_list_name("contact-list-2")
        .email_address("del-user@example.com")
        .send()
        .await;
    assert!(result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_list_contacts() {
    let client = make_ses_client().await;

    client
        .create_contact_list()
        .contact_list_name("contact-list-3")
        .send()
        .await
        .unwrap();

    client
        .create_contact()
        .contact_list_name("contact-list-3")
        .email_address("user1@example.com")
        .send()
        .await
        .unwrap();

    client
        .create_contact()
        .contact_list_name("contact-list-3")
        .email_address("user2@example.com")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_contacts()
        .contact_list_name("contact-list-3")
        .send()
        .await
        .expect("list_contacts should succeed");

    assert_eq!(resp.contacts().len(), 2);
}

// --- Dedicated IP Pool tests ---

#[tokio::test]
async fn test_create_and_get_dedicated_ip_pool() {
    let client = make_ses_client().await;

    client
        .create_dedicated_ip_pool()
        .pool_name("my-pool")
        .send()
        .await
        .expect("create_dedicated_ip_pool should succeed");

    let resp = client
        .get_dedicated_ip_pool()
        .pool_name("my-pool")
        .send()
        .await
        .expect("get_dedicated_ip_pool should succeed");

    assert_eq!(
        resp.dedicated_ip_pool()
            .map(|p| p.pool_name())
            .unwrap_or_default(),
        "my-pool"
    );
}

#[tokio::test]
async fn test_delete_dedicated_ip_pool() {
    let client = make_ses_client().await;

    client
        .create_dedicated_ip_pool()
        .pool_name("pool-to-delete")
        .send()
        .await
        .unwrap();

    client
        .delete_dedicated_ip_pool()
        .pool_name("pool-to-delete")
        .send()
        .await
        .expect("delete_dedicated_ip_pool should succeed");

    let result = client
        .get_dedicated_ip_pool()
        .pool_name("pool-to-delete")
        .send()
        .await;
    assert!(result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_list_dedicated_ip_pools() {
    let client = make_ses_client().await;

    client
        .create_dedicated_ip_pool()
        .pool_name("pool-1")
        .send()
        .await
        .unwrap();

    client
        .create_dedicated_ip_pool()
        .pool_name("pool-2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_dedicated_ip_pools()
        .send()
        .await
        .expect("list_dedicated_ip_pools should succeed");

    assert_eq!(resp.dedicated_ip_pools().len(), 2);
}

// --- Email Identity Policy tests ---

#[tokio::test]
async fn test_create_and_get_email_identity_policy() {
    let client = make_ses_client().await;

    client
        .create_email_identity()
        .email_identity("policy-test@example.com")
        .send()
        .await
        .unwrap();

    let policy_doc = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Principal":{"AWS":"*"},"Action":"ses:SendEmail","Resource":"*"}]}"#;

    client
        .create_email_identity_policy()
        .email_identity("policy-test@example.com")
        .policy_name("my-policy")
        .policy(policy_doc)
        .send()
        .await
        .expect("create_email_identity_policy should succeed");

    let resp = client
        .get_email_identity_policies()
        .email_identity("policy-test@example.com")
        .send()
        .await
        .expect("get_email_identity_policies should succeed");

    assert!(
        resp.policies()
            .map(|p| p.contains_key("my-policy"))
            .unwrap_or(false)
    );
}

#[tokio::test]
async fn test_delete_email_identity_policy() {
    let client = make_ses_client().await;

    client
        .create_email_identity()
        .email_identity("policy-del@example.com")
        .send()
        .await
        .unwrap();

    let policy_doc = r#"{"Version":"2012-10-17","Statement":[]}"#;

    client
        .create_email_identity_policy()
        .email_identity("policy-del@example.com")
        .policy_name("to-delete")
        .policy(policy_doc)
        .send()
        .await
        .unwrap();

    client
        .delete_email_identity_policy()
        .email_identity("policy-del@example.com")
        .policy_name("to-delete")
        .send()
        .await
        .expect("delete_email_identity_policy should succeed");

    let resp = client
        .get_email_identity_policies()
        .email_identity("policy-del@example.com")
        .send()
        .await
        .unwrap();

    assert!(
        !resp
            .policies()
            .map(|p| p.contains_key("to-delete"))
            .unwrap_or(false)
    );
}

#[tokio::test]
async fn test_update_email_identity_policy() {
    let client = make_ses_client().await;

    client
        .create_email_identity()
        .email_identity("policy-upd@example.com")
        .send()
        .await
        .unwrap();

    let original_policy = r#"{"Version":"2012-10-17","Statement":[]}"#;
    let updated_policy = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Deny","Principal":"*","Action":"*","Resource":"*"}]}"#;

    client
        .create_email_identity_policy()
        .email_identity("policy-upd@example.com")
        .policy_name("my-policy")
        .policy(original_policy)
        .send()
        .await
        .unwrap();

    client
        .update_email_identity_policy()
        .email_identity("policy-upd@example.com")
        .policy_name("my-policy")
        .policy(updated_policy)
        .send()
        .await
        .expect("update_email_identity_policy should succeed");

    let resp = client
        .get_email_identity_policies()
        .email_identity("policy-upd@example.com")
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp.policies()
            .and_then(|p| p.get("my-policy"))
            .map(|s| s.as_str()),
        Some(updated_policy)
    );
}

#[tokio::test]
async fn test_create_and_get_email_template() {
    let client = make_ses_client().await;

    client
        .create_email_template()
        .template_name("welcome-tmpl")
        .template_content(
            aws_sdk_sesv2::types::EmailTemplateContent::builder()
                .subject("Welcome!")
                .text("Hello, world!")
                .html("<p>Hello, world!</p>")
                .build(),
        )
        .send()
        .await
        .expect("create_email_template should succeed");

    let resp = client
        .get_email_template()
        .template_name("welcome-tmpl")
        .send()
        .await
        .expect("get_email_template should succeed");

    assert_eq!(resp.template_name(), "welcome-tmpl");
    let content = resp.template_content();
    assert_eq!(content.and_then(|c| c.subject()), Some("Welcome!"));
}

#[tokio::test]
async fn test_list_email_templates() {
    let client = make_ses_client().await;

    client
        .create_email_template()
        .template_name("tmpl-list-1")
        .template_content(
            aws_sdk_sesv2::types::EmailTemplateContent::builder()
                .subject("Subject 1")
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .create_email_template()
        .template_name("tmpl-list-2")
        .template_content(
            aws_sdk_sesv2::types::EmailTemplateContent::builder()
                .subject("Subject 2")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_email_templates()
        .send()
        .await
        .expect("list_email_templates should succeed");

    let templates = resp.templates_metadata();
    assert_eq!(templates.len(), 2);
}

#[tokio::test]
async fn test_delete_email_template() {
    let client = make_ses_client().await;

    client
        .create_email_template()
        .template_name("tmpl-del")
        .template_content(
            aws_sdk_sesv2::types::EmailTemplateContent::builder()
                .subject("Delete me")
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .delete_email_template()
        .template_name("tmpl-del")
        .send()
        .await
        .expect("delete_email_template should succeed");

    let result = client
        .get_email_template()
        .template_name("tmpl-del")
        .send()
        .await;

    assert!(result.is_err(), "Should fail after deletion");
}

#[tokio::test]
async fn test_update_email_template() {
    let client = make_ses_client().await;

    client
        .create_email_template()
        .template_name("tmpl-upd")
        .template_content(
            aws_sdk_sesv2::types::EmailTemplateContent::builder()
                .subject("Original")
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .update_email_template()
        .template_name("tmpl-upd")
        .template_content(
            aws_sdk_sesv2::types::EmailTemplateContent::builder()
                .subject("Updated")
                .build(),
        )
        .send()
        .await
        .expect("update_email_template should succeed");

    let resp = client
        .get_email_template()
        .template_name("tmpl-upd")
        .send()
        .await
        .unwrap();

    let content = resp.template_content();
    assert_eq!(content.and_then(|c| c.subject()), Some("Updated"));
}

#[tokio::test]
async fn test_create_and_get_import_job() {
    let client = make_ses_client().await;

    let resp = client
        .create_import_job()
        .import_destination(
            aws_sdk_sesv2::types::ImportDestination::builder()
                .suppression_list_destination(
                    aws_sdk_sesv2::types::SuppressionListDestination::builder()
                        .suppression_list_import_action(
                            aws_sdk_sesv2::types::SuppressionListImportAction::Put,
                        )
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .import_data_source(
            aws_sdk_sesv2::types::ImportDataSource::builder()
                .s3_url("s3://my-bucket/my-data.csv")
                .data_format(aws_sdk_sesv2::types::DataFormat::Csv)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_import_job should succeed");

    let job_id = resp.job_id().expect("Should have job_id");
    assert!(!job_id.is_empty());

    let get_resp = client
        .get_import_job()
        .job_id(job_id)
        .send()
        .await
        .expect("get_import_job should succeed");

    assert_eq!(get_resp.job_id(), Some(job_id));
    assert_eq!(
        get_resp.job_status(),
        Some(&aws_sdk_sesv2::types::JobStatus::Created)
    );
}

#[tokio::test]
async fn test_list_import_jobs() {
    let client = make_ses_client().await;

    for _ in 0..2 {
        client
            .create_import_job()
            .import_destination(
                aws_sdk_sesv2::types::ImportDestination::builder()
                    .suppression_list_destination(
                        aws_sdk_sesv2::types::SuppressionListDestination::builder()
                            .suppression_list_import_action(
                                aws_sdk_sesv2::types::SuppressionListImportAction::Put,
                            )
                            .build()
                            .unwrap(),
                    )
                    .build(),
            )
            .import_data_source(
                aws_sdk_sesv2::types::ImportDataSource::builder()
                    .s3_url("s3://bucket/data.csv")
                    .data_format(aws_sdk_sesv2::types::DataFormat::Csv)
                    .build()
                    .unwrap(),
            )
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_import_jobs()
        .send()
        .await
        .expect("list_import_jobs should succeed");

    assert_eq!(resp.import_jobs().len(), 2);
}

#[tokio::test]
async fn test_get_account() {
    let client = make_ses_client().await;

    let resp = client
        .get_account()
        .send()
        .await
        .expect("get_account should succeed");

    assert!(resp.sending_enabled());
}

#[tokio::test]
async fn test_put_account_sending_attributes() {
    let client = make_ses_client().await;

    client
        .put_account_sending_attributes()
        .sending_enabled(false)
        .send()
        .await
        .expect("put_account_sending_attributes should succeed");

    let resp = client.get_account().send().await.unwrap();
    assert!(!resp.sending_enabled());
}

#[tokio::test]
async fn test_get_and_put_deliverability_dashboard() {
    let client = make_ses_client().await;

    let resp = client
        .get_deliverability_dashboard_options()
        .send()
        .await
        .expect("get_deliverability_dashboard_options should succeed");

    assert!(!resp.dashboard_enabled());

    client
        .put_deliverability_dashboard_option()
        .dashboard_enabled(true)
        .send()
        .await
        .expect("put_deliverability_dashboard_option should succeed");

    let resp = client
        .get_deliverability_dashboard_options()
        .send()
        .await
        .unwrap();

    assert!(resp.dashboard_enabled());
}

// ── SendBulkEmail ─────────────────────────────────────────────────

#[tokio::test]
async fn test_send_bulk_email() {
    use aws_sdk_sesv2::types::{BulkEmailContent, BulkEmailEntry, Destination, Template};

    let client = make_ses_client().await;

    // Create a template to send with
    client
        .create_email_template()
        .template_name("bulk-template")
        .template_content(
            aws_sdk_sesv2::types::EmailTemplateContent::builder()
                .subject("Hello {{name}}")
                .text("Hi {{name}}")
                .build(),
        )
        .send()
        .await
        .expect("create_email_template should succeed");

    let bulk_entry = BulkEmailEntry::builder()
        .destination(
            Destination::builder()
                .to_addresses("alice@example.com")
                .build(),
        )
        .build();

    let template = Template::builder().template_name("bulk-template").build();

    let content = BulkEmailContent::builder().template(template).build();

    let resp = client
        .send_bulk_email()
        .from_email_address("sender@example.com")
        .bulk_email_entries(bulk_entry)
        .default_content(content)
        .send()
        .await
        .expect("send_bulk_email should succeed");

    let results = resp.bulk_email_entry_results();
    assert_eq!(results.len(), 1, "should have one result");
    // Each result should have a message ID
    assert!(
        results[0].message_id().is_some(),
        "result should have message_id"
    );
}

// ── Configuration Set Event Destination tests ─────────────────────

#[tokio::test]
async fn test_create_and_get_configuration_set_event_destination() {
    let client = make_ses_client().await;

    client
        .create_configuration_set()
        .configuration_set_name("cs-evt")
        .send()
        .await
        .unwrap();

    let sns_dest = aws_sdk_sesv2::types::SnsDestination::builder()
        .topic_arn("arn:aws:sns:us-east-1:123456789012:my-topic")
        .build()
        .unwrap();

    let event_dest = aws_sdk_sesv2::types::EventDestinationDefinition::builder()
        .enabled(true)
        .matching_event_types(aws_sdk_sesv2::types::EventType::Send)
        .matching_event_types(aws_sdk_sesv2::types::EventType::Bounce)
        .sns_destination(sns_dest)
        .build();

    client
        .create_configuration_set_event_destination()
        .configuration_set_name("cs-evt")
        .event_destination_name("my-dest")
        .event_destination(event_dest)
        .send()
        .await
        .expect("create event destination should succeed");

    let resp = client
        .get_configuration_set_event_destinations()
        .configuration_set_name("cs-evt")
        .send()
        .await
        .expect("get event destinations should succeed");

    assert_eq!(resp.event_destinations().len(), 1);
    assert_eq!(resp.event_destinations()[0].name(), "my-dest");
}

#[tokio::test]
async fn test_delete_configuration_set_event_destination() {
    let client = make_ses_client().await;

    client
        .create_configuration_set()
        .configuration_set_name("cs-evt-del")
        .send()
        .await
        .unwrap();

    let event_dest = aws_sdk_sesv2::types::EventDestinationDefinition::builder()
        .enabled(true)
        .matching_event_types(aws_sdk_sesv2::types::EventType::Send)
        .build();

    client
        .create_configuration_set_event_destination()
        .configuration_set_name("cs-evt-del")
        .event_destination_name("dest-to-del")
        .event_destination(event_dest)
        .send()
        .await
        .unwrap();

    client
        .delete_configuration_set_event_destination()
        .configuration_set_name("cs-evt-del")
        .event_destination_name("dest-to-del")
        .send()
        .await
        .expect("delete event destination should succeed");

    let resp = client
        .get_configuration_set_event_destinations()
        .configuration_set_name("cs-evt-del")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.event_destinations().len(), 0);
}

#[tokio::test]
async fn test_update_configuration_set_event_destination() {
    let client = make_ses_client().await;

    client
        .create_configuration_set()
        .configuration_set_name("cs-evt-upd")
        .send()
        .await
        .unwrap();

    let event_dest = aws_sdk_sesv2::types::EventDestinationDefinition::builder()
        .enabled(true)
        .matching_event_types(aws_sdk_sesv2::types::EventType::Send)
        .build();

    client
        .create_configuration_set_event_destination()
        .configuration_set_name("cs-evt-upd")
        .event_destination_name("dest-to-upd")
        .event_destination(event_dest)
        .send()
        .await
        .unwrap();

    let updated_dest = aws_sdk_sesv2::types::EventDestinationDefinition::builder()
        .enabled(false)
        .matching_event_types(aws_sdk_sesv2::types::EventType::Bounce)
        .build();

    client
        .update_configuration_set_event_destination()
        .configuration_set_name("cs-evt-upd")
        .event_destination_name("dest-to-upd")
        .event_destination(updated_dest)
        .send()
        .await
        .expect("update event destination should succeed");

    // Verify that the update succeeded (the operation itself returned 200)
    let resp = client
        .get_configuration_set_event_destinations()
        .configuration_set_name("cs-evt-upd")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.event_destinations().len(), 1);
}

// ── Custom Verification Email Template tests ──────────────────────

#[tokio::test]
async fn test_create_and_get_custom_verification_email_template() {
    let client = make_ses_client().await;

    client
        .create_custom_verification_email_template()
        .template_name("verify-tmpl")
        .from_email_address("noreply@example.com")
        .template_subject("Please verify")
        .template_content("<p>Click to verify</p>")
        .success_redirection_url("https://example.com/success")
        .failure_redirection_url("https://example.com/failure")
        .send()
        .await
        .expect("create custom verification email template should succeed");

    let resp = client
        .get_custom_verification_email_template()
        .template_name("verify-tmpl")
        .send()
        .await
        .expect("get custom verification email template should succeed");

    assert_eq!(resp.template_name().unwrap_or_default(), "verify-tmpl");
    assert_eq!(
        resp.from_email_address().unwrap_or_default(),
        "noreply@example.com"
    );
    assert_eq!(resp.template_subject().unwrap_or_default(), "Please verify");
    assert_eq!(
        resp.template_content().unwrap_or_default(),
        "<p>Click to verify</p>"
    );
    assert_eq!(
        resp.success_redirection_url().unwrap_or_default(),
        "https://example.com/success"
    );
    assert_eq!(
        resp.failure_redirection_url().unwrap_or_default(),
        "https://example.com/failure"
    );
}

#[tokio::test]
async fn test_list_custom_verification_email_templates() {
    let client = make_ses_client().await;

    client
        .create_custom_verification_email_template()
        .template_name("cv-tmpl-1")
        .from_email_address("a@example.com")
        .template_subject("Subject 1")
        .template_content("Content 1")
        .success_redirection_url("https://example.com/ok1")
        .failure_redirection_url("https://example.com/fail1")
        .send()
        .await
        .unwrap();

    client
        .create_custom_verification_email_template()
        .template_name("cv-tmpl-2")
        .from_email_address("b@example.com")
        .template_subject("Subject 2")
        .template_content("Content 2")
        .success_redirection_url("https://example.com/ok2")
        .failure_redirection_url("https://example.com/fail2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_custom_verification_email_templates()
        .send()
        .await
        .expect("list custom verification email templates should succeed");

    assert_eq!(resp.custom_verification_email_templates().len(), 2);
}

#[tokio::test]
async fn test_delete_custom_verification_email_template() {
    let client = make_ses_client().await;

    client
        .create_custom_verification_email_template()
        .template_name("cv-tmpl-del")
        .from_email_address("a@example.com")
        .template_subject("Subject")
        .template_content("Content")
        .success_redirection_url("https://example.com/ok")
        .failure_redirection_url("https://example.com/fail")
        .send()
        .await
        .unwrap();

    client
        .delete_custom_verification_email_template()
        .template_name("cv-tmpl-del")
        .send()
        .await
        .expect("delete custom verification email template should succeed");

    let result = client
        .get_custom_verification_email_template()
        .template_name("cv-tmpl-del")
        .send()
        .await;
    assert!(result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_update_custom_verification_email_template() {
    let client = make_ses_client().await;

    client
        .create_custom_verification_email_template()
        .template_name("cv-tmpl-upd")
        .from_email_address("orig@example.com")
        .template_subject("Original Subject")
        .template_content("Original Content")
        .success_redirection_url("https://example.com/ok")
        .failure_redirection_url("https://example.com/fail")
        .send()
        .await
        .unwrap();

    client
        .update_custom_verification_email_template()
        .template_name("cv-tmpl-upd")
        .from_email_address("updated@example.com")
        .template_subject("Updated Subject")
        .template_content("Updated Content")
        .success_redirection_url("https://example.com/ok-new")
        .failure_redirection_url("https://example.com/fail-new")
        .send()
        .await
        .expect("update custom verification email template should succeed");

    let resp = client
        .get_custom_verification_email_template()
        .template_name("cv-tmpl-upd")
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp.from_email_address().unwrap_or_default(),
        "updated@example.com"
    );
    assert_eq!(
        resp.template_subject().unwrap_or_default(),
        "Updated Subject"
    );
}

// ── Export Job tests ──────────────────────────────────────────────

#[tokio::test]
async fn test_create_and_get_export_job() {
    let client = make_ses_client().await;

    let resp = client
        .create_export_job()
        .export_data_source(
            aws_sdk_sesv2::types::ExportDataSource::builder()
                .metrics_data_source(
                    aws_sdk_sesv2::types::MetricsDataSource::builder()
                        .dimensions(
                            aws_sdk_sesv2::types::MetricDimensionName::EmailIdentity,
                            vec!["example.com".to_string()],
                        )
                        .metrics(
                            aws_sdk_sesv2::types::ExportMetric::builder()
                                .name(aws_sdk_sesv2::types::Metric::Send)
                                .build(),
                        )
                        .namespace(aws_sdk_sesv2::types::MetricNamespace::Vdm)
                        .start_date(aws_smithy_types::DateTime::from_secs(1700000000))
                        .end_date(aws_smithy_types::DateTime::from_secs(1700086400))
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .export_destination(
            aws_sdk_sesv2::types::ExportDestination::builder()
                .data_format(aws_sdk_sesv2::types::DataFormat::Csv)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_export_job should succeed");

    let job_id = resp.job_id().expect("should have job_id");
    assert!(!job_id.is_empty());

    let get_resp = client
        .get_export_job()
        .job_id(job_id)
        .send()
        .await
        .expect("get_export_job should succeed");

    assert_eq!(get_resp.job_id(), Some(job_id));
}

#[tokio::test]
async fn test_list_export_jobs() {
    let client = make_ses_client().await;

    for _ in 0..2 {
        client
            .create_export_job()
            .export_data_source(
                aws_sdk_sesv2::types::ExportDataSource::builder()
                    .metrics_data_source(
                        aws_sdk_sesv2::types::MetricsDataSource::builder()
                            .dimensions(
                                aws_sdk_sesv2::types::MetricDimensionName::EmailIdentity,
                                vec!["example.com".to_string()],
                            )
                            .metrics(
                                aws_sdk_sesv2::types::ExportMetric::builder()
                                    .name(aws_sdk_sesv2::types::Metric::Send)
                                    .build(),
                            )
                            .namespace(aws_sdk_sesv2::types::MetricNamespace::Vdm)
                            .start_date(aws_smithy_types::DateTime::from_secs(1700000000))
                            .end_date(aws_smithy_types::DateTime::from_secs(1700086400))
                            .build()
                            .unwrap(),
                    )
                    .build(),
            )
            .export_destination(
                aws_sdk_sesv2::types::ExportDestination::builder()
                    .data_format(aws_sdk_sesv2::types::DataFormat::Csv)
                    .build()
                    .unwrap(),
            )
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_export_jobs()
        .send()
        .await
        .expect("list_export_jobs should succeed");

    assert_eq!(resp.export_jobs().len(), 2);
}

#[tokio::test]
async fn test_cancel_export_job() {
    let client = make_ses_client().await;

    let resp = client
        .create_export_job()
        .export_data_source(
            aws_sdk_sesv2::types::ExportDataSource::builder()
                .metrics_data_source(
                    aws_sdk_sesv2::types::MetricsDataSource::builder()
                        .dimensions(
                            aws_sdk_sesv2::types::MetricDimensionName::EmailIdentity,
                            vec!["example.com".to_string()],
                        )
                        .metrics(
                            aws_sdk_sesv2::types::ExportMetric::builder()
                                .name(aws_sdk_sesv2::types::Metric::Send)
                                .build(),
                        )
                        .namespace(aws_sdk_sesv2::types::MetricNamespace::Vdm)
                        .start_date(aws_smithy_types::DateTime::from_secs(1700000000))
                        .end_date(aws_smithy_types::DateTime::from_secs(1700086400))
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .export_destination(
            aws_sdk_sesv2::types::ExportDestination::builder()
                .data_format(aws_sdk_sesv2::types::DataFormat::Csv)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let job_id = resp.job_id().unwrap();

    client
        .cancel_export_job()
        .job_id(job_id)
        .send()
        .await
        .expect("cancel_export_job should succeed");

    let get_resp = client.get_export_job().job_id(job_id).send().await.unwrap();

    assert_eq!(
        get_resp.job_status(),
        Some(&aws_sdk_sesv2::types::JobStatus::Cancelled)
    );
}

// ── Multi-Region Endpoint tests ───────────────────────────────────

#[tokio::test]
async fn test_create_and_get_multi_region_endpoint() {
    let client = make_ses_client().await;

    let route1 = aws_sdk_sesv2::types::RouteDetails::builder()
        .region("us-east-1")
        .build()
        .unwrap();
    let route2 = aws_sdk_sesv2::types::RouteDetails::builder()
        .region("eu-west-1")
        .build()
        .unwrap();

    let details = aws_sdk_sesv2::types::Details::builder()
        .routes_details(route1)
        .routes_details(route2)
        .build()
        .unwrap();

    let resp = client
        .create_multi_region_endpoint()
        .endpoint_name("my-endpoint")
        .details(details)
        .send()
        .await
        .expect("create multi-region endpoint should succeed");

    assert!(!resp.endpoint_id().unwrap_or_default().is_empty());

    let get_resp = client
        .get_multi_region_endpoint()
        .endpoint_name("my-endpoint")
        .send()
        .await
        .expect("get multi-region endpoint should succeed");

    assert_eq!(get_resp.endpoint_name().unwrap_or_default(), "my-endpoint");
}

#[tokio::test]
async fn test_list_multi_region_endpoints() {
    let client = make_ses_client().await;

    let make_details = || {
        aws_sdk_sesv2::types::Details::builder()
            .routes_details(
                aws_sdk_sesv2::types::RouteDetails::builder()
                    .region("us-east-1")
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap()
    };

    client
        .create_multi_region_endpoint()
        .endpoint_name("ep-1")
        .details(make_details())
        .send()
        .await
        .unwrap();

    client
        .create_multi_region_endpoint()
        .endpoint_name("ep-2")
        .details(make_details())
        .send()
        .await
        .unwrap();

    let resp = client
        .list_multi_region_endpoints()
        .send()
        .await
        .expect("list multi-region endpoints should succeed");

    assert_eq!(resp.multi_region_endpoints().len(), 2);
}

#[tokio::test]
async fn test_delete_multi_region_endpoint() {
    let client = make_ses_client().await;

    let details = aws_sdk_sesv2::types::Details::builder()
        .routes_details(
            aws_sdk_sesv2::types::RouteDetails::builder()
                .region("us-east-1")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    client
        .create_multi_region_endpoint()
        .endpoint_name("ep-del")
        .details(details)
        .send()
        .await
        .unwrap();

    client
        .delete_multi_region_endpoint()
        .endpoint_name("ep-del")
        .send()
        .await
        .expect("delete multi-region endpoint should succeed");

    let result = client
        .get_multi_region_endpoint()
        .endpoint_name("ep-del")
        .send()
        .await;
    assert!(result.is_err(), "get after delete should fail");
}

// ── Suppressed Destination tests ──────────────────────────────────

#[tokio::test]
async fn test_put_and_get_suppressed_destination() {
    let client = make_ses_client().await;

    client
        .put_suppressed_destination()
        .email_address("bounced@example.com")
        .reason(aws_sdk_sesv2::types::SuppressionListReason::Bounce)
        .send()
        .await
        .expect("put_suppressed_destination should succeed");

    let resp = client
        .get_suppressed_destination()
        .email_address("bounced@example.com")
        .send()
        .await
        .expect("get_suppressed_destination should succeed");

    let dest = resp.suppressed_destination().unwrap();
    assert_eq!(dest.email_address(), "bounced@example.com");
}

#[tokio::test]
async fn test_list_suppressed_destinations() {
    let client = make_ses_client().await;

    client
        .put_suppressed_destination()
        .email_address("bounce1@example.com")
        .reason(aws_sdk_sesv2::types::SuppressionListReason::Bounce)
        .send()
        .await
        .unwrap();

    client
        .put_suppressed_destination()
        .email_address("complaint1@example.com")
        .reason(aws_sdk_sesv2::types::SuppressionListReason::Complaint)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_suppressed_destinations()
        .send()
        .await
        .expect("list_suppressed_destinations should succeed");

    assert_eq!(resp.suppressed_destination_summaries().len(), 2);
}

#[tokio::test]
async fn test_delete_suppressed_destination() {
    let client = make_ses_client().await;

    client
        .put_suppressed_destination()
        .email_address("todelete@example.com")
        .reason(aws_sdk_sesv2::types::SuppressionListReason::Bounce)
        .send()
        .await
        .unwrap();

    client
        .delete_suppressed_destination()
        .email_address("todelete@example.com")
        .send()
        .await
        .expect("delete_suppressed_destination should succeed");

    let result = client
        .get_suppressed_destination()
        .email_address("todelete@example.com")
        .send()
        .await;
    assert!(result.is_err(), "get after delete should fail");
}

// ── Email Identity attribute Put tests ────────────────────────────

#[tokio::test]
async fn test_put_email_identity_configuration_set_attributes() {
    let client = make_ses_client().await;

    client
        .create_email_identity()
        .email_identity("cs-attr@example.com")
        .send()
        .await
        .unwrap();

    client
        .create_configuration_set()
        .configuration_set_name("assoc-cs")
        .send()
        .await
        .unwrap();

    client
        .put_email_identity_configuration_set_attributes()
        .email_identity("cs-attr@example.com")
        .configuration_set_name("assoc-cs")
        .send()
        .await
        .expect("put email identity configuration set attributes should succeed");
}

#[tokio::test]
async fn test_put_email_identity_dkim_attributes() {
    let client = make_ses_client().await;

    client
        .create_email_identity()
        .email_identity("dkim@example.com")
        .send()
        .await
        .unwrap();

    client
        .put_email_identity_dkim_attributes()
        .email_identity("dkim@example.com")
        .signing_enabled(true)
        .send()
        .await
        .expect("put email identity dkim attributes should succeed");
}

#[tokio::test]
async fn test_put_email_identity_dkim_signing_attributes() {
    let client = make_ses_client().await;

    client
        .create_email_identity()
        .email_identity("dkim-sign@example.com")
        .send()
        .await
        .unwrap();

    let signing_attrs = aws_sdk_sesv2::types::DkimSigningAttributes::builder()
        .domain_signing_selector("selector1")
        .next_signing_key_length(aws_sdk_sesv2::types::DkimSigningKeyLength::Rsa2048Bit)
        .build();

    client
        .put_email_identity_dkim_signing_attributes()
        .email_identity("dkim-sign@example.com")
        .signing_attributes_origin(aws_sdk_sesv2::types::DkimSigningAttributesOrigin::External)
        .signing_attributes(signing_attrs)
        .send()
        .await
        .expect("put email identity dkim signing attributes should succeed");
}

#[tokio::test]
async fn test_put_email_identity_feedback_attributes() {
    let client = make_ses_client().await;

    client
        .create_email_identity()
        .email_identity("feedback@example.com")
        .send()
        .await
        .unwrap();

    client
        .put_email_identity_feedback_attributes()
        .email_identity("feedback@example.com")
        .email_forwarding_enabled(false)
        .send()
        .await
        .expect("put email identity feedback attributes should succeed");
}

#[tokio::test]
async fn test_put_email_identity_mail_from_attributes() {
    let client = make_ses_client().await;

    client
        .create_email_identity()
        .email_identity("example.com")
        .send()
        .await
        .unwrap();

    client
        .put_email_identity_mail_from_attributes()
        .email_identity("example.com")
        .mail_from_domain("mail.example.com")
        .behavior_on_mx_failure(aws_sdk_sesv2::types::BehaviorOnMxFailure::UseDefaultValue)
        .send()
        .await
        .expect("put email identity mail from attributes should succeed");
}

// ── Account attribute Put tests ───────────────────────────────────

#[tokio::test]
async fn test_put_account_details() {
    let client = make_ses_client().await;

    client
        .put_account_details()
        .mail_type(aws_sdk_sesv2::types::MailType::Marketing)
        .website_url("https://example.com")
        .use_case_description("Test use case")
        .send()
        .await
        .expect("put_account_details should succeed");
}

#[tokio::test]
async fn test_put_account_dedicated_ip_warmup_attributes() {
    let client = make_ses_client().await;

    client
        .put_account_dedicated_ip_warmup_attributes()
        .auto_warmup_enabled(true)
        .send()
        .await
        .expect("put_account_dedicated_ip_warmup_attributes should succeed");

    let resp = client.get_account().send().await.unwrap();
    assert!(resp.dedicated_ip_auto_warmup_enabled());
}

#[tokio::test]
async fn test_put_account_suppression_attributes() {
    let client = make_ses_client().await;

    client
        .put_account_suppression_attributes()
        .suppressed_reasons(aws_sdk_sesv2::types::SuppressionListReason::Bounce)
        .suppressed_reasons(aws_sdk_sesv2::types::SuppressionListReason::Complaint)
        .send()
        .await
        .expect("put_account_suppression_attributes should succeed");
}

#[tokio::test]
async fn test_put_account_vdm_attributes() {
    let client = make_ses_client().await;

    let vdm = aws_sdk_sesv2::types::VdmAttributes::builder()
        .vdm_enabled(aws_sdk_sesv2::types::FeatureStatus::Enabled)
        .build()
        .unwrap();

    client
        .put_account_vdm_attributes()
        .vdm_attributes(vdm)
        .send()
        .await
        .expect("put_account_vdm_attributes should succeed");
}

// ── Dedicated IP Pool scaling attributes test ─────────────────────

#[tokio::test]
async fn test_put_dedicated_ip_pool_scaling_attributes() {
    let client = make_ses_client().await;

    client
        .create_dedicated_ip_pool()
        .pool_name("scale-pool")
        .send()
        .await
        .unwrap();

    client
        .put_dedicated_ip_pool_scaling_attributes()
        .pool_name("scale-pool")
        .scaling_mode(aws_sdk_sesv2::types::ScalingMode::Managed)
        .send()
        .await
        .expect("put_dedicated_ip_pool_scaling_attributes should succeed");
}

// ── Configuration Set option Put tests ────────────────────────────

#[tokio::test]
async fn test_put_configuration_set_delivery_options() {
    let client = make_ses_client().await;

    client
        .create_configuration_set()
        .configuration_set_name("cs-delivery")
        .send()
        .await
        .unwrap();

    client
        .put_configuration_set_delivery_options()
        .configuration_set_name("cs-delivery")
        .tls_policy(aws_sdk_sesv2::types::TlsPolicy::Require)
        .send()
        .await
        .expect("put configuration set delivery options should succeed");
}

#[tokio::test]
async fn test_put_configuration_set_reputation_options() {
    let client = make_ses_client().await;

    client
        .create_configuration_set()
        .configuration_set_name("cs-rep")
        .send()
        .await
        .unwrap();

    client
        .put_configuration_set_reputation_options()
        .configuration_set_name("cs-rep")
        .reputation_metrics_enabled(true)
        .send()
        .await
        .expect("put configuration set reputation options should succeed");
}

#[tokio::test]
async fn test_put_configuration_set_sending_options() {
    let client = make_ses_client().await;

    client
        .create_configuration_set()
        .configuration_set_name("cs-send")
        .send()
        .await
        .unwrap();

    client
        .put_configuration_set_sending_options()
        .configuration_set_name("cs-send")
        .sending_enabled(true)
        .send()
        .await
        .expect("put configuration set sending options should succeed");
}

#[tokio::test]
async fn test_put_configuration_set_suppression_options() {
    let client = make_ses_client().await;

    client
        .create_configuration_set()
        .configuration_set_name("cs-supp")
        .send()
        .await
        .unwrap();

    client
        .put_configuration_set_suppression_options()
        .configuration_set_name("cs-supp")
        .suppressed_reasons(aws_sdk_sesv2::types::SuppressionListReason::Bounce)
        .send()
        .await
        .expect("put configuration set suppression options should succeed");
}

#[tokio::test]
async fn test_put_configuration_set_tracking_options() {
    let client = make_ses_client().await;

    client
        .create_configuration_set()
        .configuration_set_name("cs-track")
        .send()
        .await
        .unwrap();

    client
        .put_configuration_set_tracking_options()
        .configuration_set_name("cs-track")
        .custom_redirect_domain("track.example.com")
        .send()
        .await
        .expect("put configuration set tracking options should succeed");
}

#[tokio::test]
async fn test_put_configuration_set_vdm_options() {
    let client = make_ses_client().await;

    client
        .create_configuration_set()
        .configuration_set_name("cs-vdm")
        .send()
        .await
        .unwrap();

    let vdm_options = aws_sdk_sesv2::types::VdmOptions::builder().build();

    client
        .put_configuration_set_vdm_options()
        .configuration_set_name("cs-vdm")
        .vdm_options(vdm_options)
        .send()
        .await
        .expect("put configuration set vdm options should succeed");
}

// ── Update Contact / Contact List tests ───────────────────────────

#[tokio::test]
async fn test_update_contact_list() {
    let client = make_ses_client().await;

    client
        .create_contact_list()
        .contact_list_name("updatable-list")
        .send()
        .await
        .unwrap();

    client
        .update_contact_list()
        .contact_list_name("updatable-list")
        .description("Updated description")
        .send()
        .await
        .expect("update_contact_list should succeed");

    let resp = client
        .get_contact_list()
        .contact_list_name("updatable-list")
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp.description().unwrap_or_default(),
        "Updated description"
    );
}

#[tokio::test]
async fn test_update_contact() {
    let client = make_ses_client().await;

    client
        .create_contact_list()
        .contact_list_name("upd-contact-list")
        .send()
        .await
        .unwrap();

    client
        .create_contact()
        .contact_list_name("upd-contact-list")
        .email_address("contact@example.com")
        .send()
        .await
        .unwrap();

    client
        .update_contact()
        .contact_list_name("upd-contact-list")
        .email_address("contact@example.com")
        .unsubscribe_all(true)
        .send()
        .await
        .expect("update_contact should succeed");

    let resp = client
        .get_contact()
        .contact_list_name("upd-contact-list")
        .email_address("contact@example.com")
        .send()
        .await
        .unwrap();

    assert!(resp.unsubscribe_all());
}

// ── Tag resource tests ────────────────────────────────────────────

#[tokio::test]
async fn test_tag_and_list_tags_for_resource() {
    let client = make_ses_client().await;

    client
        .create_configuration_set()
        .configuration_set_name("tagged-cs")
        .send()
        .await
        .unwrap();

    let tag1 = aws_sdk_sesv2::types::Tag::builder()
        .key("env")
        .value("test")
        .build()
        .unwrap();
    let tag2 = aws_sdk_sesv2::types::Tag::builder()
        .key("team")
        .value("platform")
        .build()
        .unwrap();

    client
        .tag_resource()
        .resource_arn("arn:aws:ses:us-east-1:123456789012:configuration-set/tagged-cs")
        .tags(tag1)
        .tags(tag2)
        .send()
        .await
        .expect("tag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn("arn:aws:ses:us-east-1:123456789012:configuration-set/tagged-cs")
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    assert_eq!(resp.tags().len(), 2);
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_ses_client().await;

    client
        .create_configuration_set()
        .configuration_set_name("untag-cs")
        .send()
        .await
        .unwrap();

    let tag = aws_sdk_sesv2::types::Tag::builder()
        .key("env")
        .value("test")
        .build()
        .unwrap();

    client
        .tag_resource()
        .resource_arn("arn:aws:ses:us-east-1:123456789012:configuration-set/untag-cs")
        .tags(tag)
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource_arn("arn:aws:ses:us-east-1:123456789012:configuration-set/untag-cs")
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn("arn:aws:ses:us-east-1:123456789012:configuration-set/untag-cs")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.tags().len(), 0);
}

// ── Duplicate / NotFound error cases ──────────────────────────────

#[tokio::test]
async fn test_create_duplicate_configuration_set_fails() {
    let client = make_ses_client().await;

    client
        .create_configuration_set()
        .configuration_set_name("dup-cs")
        .send()
        .await
        .unwrap();

    let result = client
        .create_configuration_set()
        .configuration_set_name("dup-cs")
        .send()
        .await;
    assert!(result.is_err(), "duplicate configuration set should fail");
}

#[tokio::test]
async fn test_get_nonexistent_configuration_set_fails() {
    let client = make_ses_client().await;

    let result = client
        .get_configuration_set()
        .configuration_set_name("no-such-cs")
        .send()
        .await;
    assert!(result.is_err(), "get nonexistent config set should fail");
}

#[tokio::test]
async fn test_create_duplicate_contact_list_fails() {
    let client = make_ses_client().await;

    client
        .create_contact_list()
        .contact_list_name("dup-list")
        .send()
        .await
        .unwrap();

    let result = client
        .create_contact_list()
        .contact_list_name("dup-list")
        .send()
        .await;
    assert!(result.is_err(), "duplicate contact list should fail");
}

#[tokio::test]
async fn test_create_duplicate_dedicated_ip_pool_fails() {
    let client = make_ses_client().await;

    client
        .create_dedicated_ip_pool()
        .pool_name("dup-pool")
        .send()
        .await
        .unwrap();

    let result = client
        .create_dedicated_ip_pool()
        .pool_name("dup-pool")
        .send()
        .await;
    assert!(result.is_err(), "duplicate IP pool should fail");
}

#[tokio::test]
async fn test_create_duplicate_email_template_fails() {
    let client = make_ses_client().await;

    client
        .create_email_template()
        .template_name("dup-tmpl")
        .template_content(
            aws_sdk_sesv2::types::EmailTemplateContent::builder()
                .subject("Sub")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let result = client
        .create_email_template()
        .template_name("dup-tmpl")
        .template_content(
            aws_sdk_sesv2::types::EmailTemplateContent::builder()
                .subject("Sub")
                .build(),
        )
        .send()
        .await;
    assert!(result.is_err(), "duplicate email template should fail");
}

#[tokio::test]
async fn test_get_nonexistent_email_template_fails() {
    let client = make_ses_client().await;

    let result = client
        .get_email_template()
        .template_name("no-such-tmpl")
        .send()
        .await;
    assert!(result.is_err(), "get nonexistent template should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_configuration_set_fails() {
    let client = make_ses_client().await;

    let result = client
        .delete_configuration_set()
        .configuration_set_name("no-such-cs")
        .send()
        .await;
    assert!(result.is_err(), "delete nonexistent config set should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_contact_list_fails() {
    let client = make_ses_client().await;

    let result = client
        .delete_contact_list()
        .contact_list_name("no-such-list")
        .send()
        .await;
    assert!(
        result.is_err(),
        "delete nonexistent contact list should fail"
    );
}

#[tokio::test]
async fn test_get_nonexistent_suppressed_destination_fails() {
    let client = make_ses_client().await;

    let result = client
        .get_suppressed_destination()
        .email_address("nosuch@example.com")
        .send()
        .await;
    assert!(
        result.is_err(),
        "get nonexistent suppressed destination should fail"
    );
}

// ── Stub handler smoke tests ──────────────────────────────────────

#[tokio::test]
async fn test_batch_get_metric_data() {
    let client = make_ses_client().await;

    let resp = client.batch_get_metric_data().send().await;
    // Stub handler: should succeed (returns 200 with empty body)
    assert!(resp.is_ok(), "batch_get_metric_data should succeed");
}

#[tokio::test]
async fn test_create_deliverability_test_report() {
    let client = make_ses_client().await;

    let content = aws_sdk_sesv2::types::EmailContent::builder()
        .simple(
            aws_sdk_sesv2::types::Message::builder()
                .subject(
                    aws_sdk_sesv2::types::Content::builder()
                        .data("Test")
                        .build()
                        .unwrap(),
                )
                .body(aws_sdk_sesv2::types::Body::builder().build())
                .build(),
        )
        .build();

    let resp = client
        .create_deliverability_test_report()
        .from_email_address("test@example.com")
        .content(content)
        .send()
        .await;
    assert!(
        resp.is_ok(),
        "create_deliverability_test_report stub should succeed"
    );
}

#[tokio::test]
async fn test_get_blacklist_reports() {
    let client = make_ses_client().await;

    let resp = client
        .get_blacklist_reports()
        .blacklist_item_names("192.0.2.1")
        .send()
        .await;
    assert!(resp.is_ok(), "get_blacklist_reports stub should succeed");
}

#[tokio::test]
async fn test_list_deliverability_test_reports() {
    let client = make_ses_client().await;

    let resp = client.list_deliverability_test_reports().send().await;
    assert!(
        resp.is_ok(),
        "list_deliverability_test_reports stub should succeed"
    );
}

#[tokio::test]
async fn test_send_custom_verification_email() {
    let client = make_ses_client().await;

    // Create the template first
    client
        .create_custom_verification_email_template()
        .template_name("verify-tmpl")
        .from_email_address("no-reply@example.com")
        .template_subject("Verify your email")
        .template_content("<p>Please verify</p>")
        .success_redirection_url("https://example.com/success")
        .failure_redirection_url("https://example.com/failure")
        .send()
        .await
        .expect("create_custom_verification_email_template should succeed");

    let resp = client
        .send_custom_verification_email()
        .email_address("verify@example.com")
        .template_name("verify-tmpl")
        .send()
        .await;
    assert!(
        resp.is_ok(),
        "send_custom_verification_email should succeed"
    );

    // Sending with a non-existent template should fail
    let bad_resp = client
        .send_custom_verification_email()
        .email_address("verify@example.com")
        .template_name("nonexistent-template")
        .send()
        .await;
    assert!(
        bad_resp.is_err(),
        "send_custom_verification_email with missing template should fail"
    );
}

#[tokio::test]
async fn test_list_recommendations() {
    let client = make_ses_client().await;

    let resp = client.list_recommendations().send().await;
    assert!(resp.is_ok(), "list_recommendations stub should succeed");
}

#[tokio::test]
async fn test_get_deliverability_test_report() {
    let client = make_ses_client().await;

    let content = aws_sdk_sesv2::types::EmailContent::builder()
        .simple(
            aws_sdk_sesv2::types::Message::builder()
                .subject(
                    aws_sdk_sesv2::types::Content::builder()
                        .data("Subject")
                        .build()
                        .unwrap(),
                )
                .body(aws_sdk_sesv2::types::Body::builder().build())
                .build(),
        )
        .build();

    let create_resp = client
        .create_deliverability_test_report()
        .from_email_address("sender@example.com")
        .content(content)
        .send()
        .await
        .expect("create_deliverability_test_report should succeed");

    let report_id = create_resp.report_id();

    let get_resp = client
        .get_deliverability_test_report()
        .report_id(report_id)
        .send()
        .await
        .expect("get_deliverability_test_report should succeed");

    let report = get_resp.deliverability_test_report().unwrap();
    assert_eq!(report.from_email_address(), Some("sender@example.com"));

    // Non-existent report should fail
    let bad_resp = client
        .get_deliverability_test_report()
        .report_id("nonexistent-id")
        .send()
        .await;
    assert!(
        bad_resp.is_err(),
        "get_deliverability_test_report with unknown id should fail"
    );
}

#[tokio::test]
async fn test_test_render_email_template() {
    let client = make_ses_client().await;

    // Create a template first
    let tmpl_content = aws_sdk_sesv2::types::EmailTemplateContent::builder()
        .subject("Hello {{name}}")
        .html("<p>Hello {{name}}</p>")
        .text("Hello {{name}}")
        .build();
    client
        .create_email_template()
        .template_name("render-test-tmpl")
        .template_content(tmpl_content)
        .send()
        .await
        .expect("create_email_template should succeed");

    let resp = client
        .test_render_email_template()
        .template_name("render-test-tmpl")
        .template_data(r#"{"name":"World"}"#)
        .send()
        .await
        .expect("test_render_email_template should succeed");

    // rendered_template() returns &str — just check it's accessible
    let _ = resp.rendered_template();

    // Non-existent template should fail
    let bad_resp = client
        .test_render_email_template()
        .template_name("no-such-template")
        .template_data("{}")
        .send()
        .await;
    assert!(
        bad_resp.is_err(),
        "test_render_email_template with missing template should fail"
    );
}

#[tokio::test]
async fn test_get_message_insights() {
    let client = make_ses_client().await;

    // Send an email first
    let to_addr = aws_sdk_sesv2::types::Destination::builder()
        .to_addresses("recipient@example.com")
        .build();
    let send_resp = client
        .send_email()
        .from_email_address("sender@example.com")
        .destination(to_addr)
        .content(
            aws_sdk_sesv2::types::EmailContent::builder()
                .simple(
                    aws_sdk_sesv2::types::Message::builder()
                        .subject(
                            aws_sdk_sesv2::types::Content::builder()
                                .data("Hello")
                                .build()
                                .unwrap(),
                        )
                        .body(aws_sdk_sesv2::types::Body::builder().build())
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .expect("send_email should succeed");

    let message_id = send_resp.message_id().unwrap_or_default();

    let resp = client
        .get_message_insights()
        .message_id(message_id)
        .send()
        .await
        .expect("get_message_insights should succeed");

    assert_eq!(resp.from_email_address(), Some("sender@example.com"));

    // Non-existent message should fail
    let bad_resp = client
        .get_message_insights()
        .message_id("no-such-message")
        .send()
        .await;
    assert!(
        bad_resp.is_err(),
        "get_message_insights with unknown message_id should fail"
    );
}

#[tokio::test]
async fn test_get_email_address_insights() {
    let client = make_ses_client().await;

    let resp = client
        .get_email_address_insights()
        .email_address("user@example.com")
        .send()
        .await;
    assert!(resp.is_ok(), "get_email_address_insights should succeed");
}

#[tokio::test]
async fn test_reputation_entities_lifecycle() {
    use aws_sdk_sesv2::types::ReputationEntityType;
    let client = make_ses_client().await;

    // Initially no entities
    let list_resp = client
        .list_reputation_entities()
        .send()
        .await
        .expect("list_reputation_entities should succeed");
    assert!(
        list_resp.reputation_entities().is_empty(),
        "no reputation entities initially"
    );

    // Update customer-managed status to create an entity
    client
        .update_reputation_entity_customer_managed_status()
        .reputation_entity_type(ReputationEntityType::Resource)
        .reputation_entity_reference("test@example.com")
        .sending_status(aws_sdk_sesv2::types::SendingStatus::Disabled)
        .send()
        .await
        .expect("update_reputation_entity_customer_managed_status should succeed");

    // Should now appear in list
    let list_resp2 = client
        .list_reputation_entities()
        .send()
        .await
        .expect("list_reputation_entities should succeed after update");
    assert_eq!(
        list_resp2.reputation_entities().len(),
        1,
        "one reputation entity after update"
    );

    // Get it by type and reference
    let get_resp = client
        .get_reputation_entity()
        .reputation_entity_type(ReputationEntityType::Resource)
        .reputation_entity_reference("test@example.com")
        .send()
        .await
        .expect("get_reputation_entity should succeed");
    let entity = get_resp.reputation_entity().unwrap();
    assert_eq!(
        entity.reputation_entity_reference(),
        Some("test@example.com")
    );

    // Update policy (supply a fake ARN as the policy field expects an ARN)
    client
        .update_reputation_entity_policy()
        .reputation_entity_type(ReputationEntityType::Resource)
        .reputation_entity_reference("test@example.com")
        .reputation_entity_policy("arn:aws:ses:us-east-1:123456789012:reputation-policy/standard")
        .send()
        .await
        .expect("update_reputation_entity_policy should succeed");

    // Get non-existent entity should fail
    let bad_resp: Result<_, _> = client
        .get_reputation_entity()
        .reputation_entity_type(ReputationEntityType::Resource)
        .reputation_entity_reference("nosuchaddr@example.com")
        .send()
        .await;
    assert!(
        bad_resp.is_err(),
        "get_reputation_entity for missing entity should fail"
    );
}
