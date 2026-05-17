# winterbaume-ses

SES v1 (Simple Email Service) mock implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | SES v1 |
| AWS model | `ses` |
| Protocol | awsQuery |
| winterbaume coverage | 38/71 operations (53.5%) |
| stubs (routed, returns empty/default) | 2/71 operations (2.8%) |
| moto coverage | 38/71 operations (53.5%) |
| floci coverage | 0/71 operations (0.0%) |
| kumo coverage | 0/71 operations (0.0%) |
| Coverage report date | 2026-05-17 |

## Server-mode usage

Install `winterbaume-server` from crates.io or run it from a workspace checkout, then point the AWS CLI at it:

```sh
# Installed binary ( from crates.io ):
cargo install winterbaume-server
winterbaume-server --host 127.0.0.1 --port 5555

# Or, from a workspace checkout:
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws ses list-identities
```

## Example

```rust
use std::sync::Arc;

use aws_sdk_ses::config::BehaviorVersion;
use winterbaume_core::{MockAws, StatefulService};
use winterbaume_ses::SesService;

#[tokio::main]
async fn main() {
    // Keep an Arc<SesService> so we can inspect the mock state after the
    // SendEmail call. The mock builder accepts the Arc directly via the
    // blanket `MockService for Arc<T>` impl in winterbaume-core.
    let svc = Arc::new(SesService::new());
    let mock = MockAws::builder().with_service(Arc::clone(&svc)).build();

    let region = "us-east-1";
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ses::config::Region::new(region))
        .load()
        .await;

    let client = aws_sdk_ses::Client::from_conf(
        aws_sdk_ses::config::Builder::from(&config)
            .endpoint_url("https://ses.us-east-1.amazonaws.com")
            .build(),
    );

    // Verify the sender so the mock has at least one identity registered.
    client
        .verify_email_identity()
        .email_address("sender@example.com")
        .send()
        .await
        .expect("verify_email_identity should succeed");

    let identities = client
        .list_identities()
        .send()
        .await
        .expect("list_identities should succeed");
    println!("SES v1 identities: {:?}", identities.identities());

    // Send a multipart-style email with both text and HTML bodies.
    let send_resp = client
        .send_email()
        .source("sender@example.com")
        .destination(
            aws_sdk_ses::types::Destination::builder()
                .to_addresses("alice@example.com")
                .cc_addresses("carbon@example.com")
                .build(),
        )
        .message(
            aws_sdk_ses::types::Message::builder()
                .subject(
                    aws_sdk_ses::types::Content::builder()
                        .data("Welcome to winterbaume")
                        .build()
                        .unwrap(),
                )
                .body(
                    aws_sdk_ses::types::Body::builder()
                        .text(
                            aws_sdk_ses::types::Content::builder()
                                .data("Hello from the SES v1 mock.")
                                .build()
                                .unwrap(),
                        )
                        .html(
                            aws_sdk_ses::types::Content::builder()
                                .data("<p>Hello from the <b>SES v1</b> mock.</p>")
                                .build()
                                .unwrap(),
                        )
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .expect("send_email should succeed");
    let message_id = send_resp.message_id().to_string();
    println!("SES v1 SendEmail message ID: {message_id}");

    // SES v1 has no public API to read individual sent messages, so the
    // mock surfaces them through the snapshot view instead.
    let snapshot = svc.snapshot(mock.account_id(), region).await;
    let sent = snapshot
        .sent_emails
        .iter()
        .find(|e| e.message_id == message_id)
        .expect("sent email should be recorded in mock state");

    assert_eq!(sent.source, "sender@example.com");
    assert_eq!(sent.to_addresses, vec!["alice@example.com".to_string()]);
    assert_eq!(sent.cc_addresses, vec!["carbon@example.com".to_string()]);
    assert_eq!(sent.subject, "Welcome to winterbaume");
    assert_eq!(
        sent.text_body.as_deref(),
        Some("Hello from the SES v1 mock.")
    );
    assert_eq!(
        sent.html_body.as_deref(),
        Some("<p>Hello from the <b>SES v1</b> mock.</p>")
    );

    println!(
        "SES v1 verified sent email: from={} to={:?} subject={:?}",
        sent.source, sent.to_addresses, sent.subject
    );
}
```

## Implemented APIs (38)

- `CloneReceiptRuleSet`
- `CreateConfigurationSet`
- `CreateConfigurationSetEventDestination`
- `CreateReceiptRule`
- `CreateReceiptRuleSet`
- `CreateTemplate`
- `DeleteConfigurationSet`
- `DeleteIdentity`
- `DeleteReceiptRuleSet`
- `DeleteTemplate`
- `DescribeActiveReceiptRuleSet`
- `DescribeConfigurationSet`
- `DescribeReceiptRule`
- `DescribeReceiptRuleSet`
- `GetIdentityDkimAttributes`
- `GetIdentityMailFromDomainAttributes`
- `GetIdentityNotificationAttributes`
- `GetIdentityVerificationAttributes`
- `GetTemplate`
- `ListConfigurationSets`
- `ListIdentities`
- `ListReceiptRuleSets`
- `ListTemplates`
- `ListVerifiedEmailAddresses`
- `SendBulkTemplatedEmail`
- `SendEmail`
- `SendRawEmail`
- `SendTemplatedEmail`
- `SetActiveReceiptRuleSet`
- `SetIdentityFeedbackForwardingEnabled`
- `SetIdentityMailFromDomain`
- `SetIdentityNotificationTopic`
- `UpdateConfigurationSetReputationMetricsEnabled`
- `UpdateReceiptRule`
- `UpdateTemplate`
- `VerifyDomainIdentity`
- `VerifyEmailAddress`
- `VerifyEmailIdentity`

<details><summary>Stubbed APIs (2) &mdash; routed but return an empty/default response</summary>

- `GetSendQuota`
- `GetSendStatistics`

</details>

<details><summary>Not yet implemented APIs (31)</summary>

- `CreateConfigurationSetTrackingOptions`
- `CreateCustomVerificationEmailTemplate`
- `CreateReceiptFilter`
- `DeleteConfigurationSetEventDestination`
- `DeleteConfigurationSetTrackingOptions`
- `DeleteCustomVerificationEmailTemplate`
- `DeleteIdentityPolicy`
- `DeleteReceiptFilter`
- `DeleteReceiptRule`
- `DeleteVerifiedEmailAddress`
- `GetAccountSendingEnabled`
- `GetCustomVerificationEmailTemplate`
- `GetIdentityPolicies`
- `ListCustomVerificationEmailTemplates`
- `ListIdentityPolicies`
- `ListReceiptFilters`
- `PutConfigurationSetDeliveryOptions`
- `PutIdentityPolicy`
- `ReorderReceiptRuleSet`
- `SendBounce`
- `SendCustomVerificationEmail`
- `SetIdentityDkimEnabled`
- `SetIdentityHeadersInNotificationsEnabled`
- `SetReceiptRulePosition`
- `TestRenderTemplate`
- `UpdateAccountSendingEnabled`
- `UpdateConfigurationSetEventDestination`
- `UpdateConfigurationSetSendingEnabled`
- `UpdateConfigurationSetTrackingOptions`
- `UpdateCustomVerificationEmailTemplate`
- `VerifyDomainDkim`

</details>
