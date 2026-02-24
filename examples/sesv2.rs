//! Example: SES v2
//!
//! Demonstrates using aws-sdk-sesv2 with winterbaume, including a SendEmail
//! call and verification of the email contents recorded by the mock.
//!
//! Run with:
//!   cargo run --example sesv2 --package winterbaume-examples

use std::sync::Arc;

use aws_sdk_sesv2::config::BehaviorVersion;
use winterbaume_core::{MockAws, StatefulService};
use winterbaume_sesv2::SesV2Service;

#[tokio::main]
async fn main() {
    // Keep an Arc<SesV2Service> so we can inspect the mock state after the
    // SendEmail call. The mock builder accepts the Arc directly via the
    // blanket `MockService for Arc<T>` impl in winterbaume-core.
    let svc = Arc::new(SesV2Service::new());
    let mock = MockAws::builder().with_service(Arc::clone(&svc)).build();

    let region = "us-east-1";
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_sesv2::config::Region::new(region))
        .load()
        .await;

    let client = aws_sdk_sesv2::Client::new(&config);

    client
        .create_email_identity()
        .email_identity("sender@example.com")
        .send()
        .await
        .expect("create_email_identity should succeed");

    let identities = client
        .list_email_identities()
        .send()
        .await
        .expect("list_email_identities should succeed");
    println!(
        "SES v2 email identities: {}",
        identities.email_identities().len()
    );

    let send_resp = client
        .send_email()
        .from_email_address("sender@example.com")
        .destination(
            aws_sdk_sesv2::types::Destination::builder()
                .to_addresses("alice@example.com")
                .build(),
        )
        .content(
            aws_sdk_sesv2::types::EmailContent::builder()
                .simple(
                    aws_sdk_sesv2::types::Message::builder()
                        .subject(
                            aws_sdk_sesv2::types::Content::builder()
                                .data("Welcome to winterbaume")
                                .build()
                                .unwrap(),
                        )
                        .body(
                            aws_sdk_sesv2::types::Body::builder()
                                .text(
                                    aws_sdk_sesv2::types::Content::builder()
                                        .data("Hello from the SES v2 mock.")
                                        .build()
                                        .unwrap(),
                                )
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .expect("send_email should succeed");
    let message_id = send_resp
        .message_id()
        .expect("send_email should return a message id")
        .to_string();
    println!("SES v2 SendEmail message ID: {message_id}");

    // Verify the email through the mock state snapshot.
    let snapshot = svc.snapshot(mock.account_id(), region).await;
    let sent = snapshot
        .sent_emails
        .iter()
        .find(|e| e.message_id == message_id)
        .expect("sent email should be recorded in mock state");

    assert_eq!(sent.from, "sender@example.com");
    assert_eq!(sent.to, vec!["alice@example.com".to_string()]);
    assert_eq!(sent.subject, "Welcome to winterbaume");
    assert_eq!(sent.body, "Hello from the SES v2 mock.");

    println!(
        "SES v2 verified sent email: from={} to={:?} subject={:?}",
        sent.from, sent.to, sent.subject
    );
}
