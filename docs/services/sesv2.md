# winterbaume-sesv2

SES v2 service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | SES v2 |
| AWS model | `sesv2` |
| Protocol | restJson1 |
| winterbaume coverage | 106/110 operations (96.4%) |
| stubs (routed, returns empty/default) | 4/110 operations (3.6%) |
| moto coverage | 28/110 operations (25.5%) |
| floci coverage | 0/110 operations (0.0%) |
| kumo coverage | 9/110 operations (8.2%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws sesv2 list-email-identities
```

## Example

```rust
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
```

## Implemented APIs (106)

- `CancelExportJob`
- `CreateConfigurationSet`
- `CreateConfigurationSetEventDestination`
- `CreateContact`
- `CreateContactList`
- `CreateCustomVerificationEmailTemplate`
- `CreateDedicatedIpPool`
- `CreateDeliverabilityTestReport`
- `CreateEmailIdentity`
- `CreateEmailIdentityPolicy`
- `CreateEmailTemplate`
- `CreateExportJob`
- `CreateImportJob`
- `CreateMultiRegionEndpoint`
- `CreateTenant`
- `CreateTenantResourceAssociation`
- `DeleteConfigurationSet`
- `DeleteConfigurationSetEventDestination`
- `DeleteContact`
- `DeleteContactList`
- `DeleteCustomVerificationEmailTemplate`
- `DeleteDedicatedIpPool`
- `DeleteEmailIdentity`
- `DeleteEmailIdentityPolicy`
- `DeleteEmailTemplate`
- `DeleteMultiRegionEndpoint`
- `DeleteSuppressedDestination`
- `DeleteTenant`
- `DeleteTenantResourceAssociation`
- `GetAccount`
- `GetBlacklistReports`
- `GetConfigurationSet`
- `GetConfigurationSetEventDestinations`
- `GetContact`
- `GetContactList`
- `GetCustomVerificationEmailTemplate`
- `GetDedicatedIp`
- `GetDedicatedIpPool`
- `GetDedicatedIps`
- `GetDeliverabilityDashboardOptions`
- `GetDeliverabilityTestReport`
- `GetDomainDeliverabilityCampaign`
- `GetEmailIdentity`
- `GetEmailIdentityPolicies`
- `GetEmailTemplate`
- `GetExportJob`
- `GetImportJob`
- `GetMessageInsights`
- `GetMultiRegionEndpoint`
- `GetReputationEntity`
- `GetSuppressedDestination`
- `GetTenant`
- `ListConfigurationSets`
- `ListContactLists`
- `ListContacts`
- `ListCustomVerificationEmailTemplates`
- `ListDedicatedIpPools`
- `ListDeliverabilityTestReports`
- `ListDomainDeliverabilityCampaigns`
- `ListEmailIdentities`
- `ListEmailTemplates`
- `ListExportJobs`
- `ListImportJobs`
- `ListMultiRegionEndpoints`
- `ListReputationEntities`
- `ListResourceTenants`
- `ListSuppressedDestinations`
- `ListTagsForResource`
- `ListTenantResources`
- `ListTenants`
- `PutAccountDedicatedIpWarmupAttributes`
- `PutAccountDetails`
- `PutAccountSendingAttributes`
- `PutAccountSuppressionAttributes`
- `PutAccountVdmAttributes`
- `PutConfigurationSetArchivingOptions`
- `PutConfigurationSetDeliveryOptions`
- `PutConfigurationSetReputationOptions`
- `PutConfigurationSetSendingOptions`
- `PutConfigurationSetSuppressionOptions`
- `PutConfigurationSetTrackingOptions`
- `PutConfigurationSetVdmOptions`
- `PutDedicatedIpInPool`
- `PutDedicatedIpPoolScalingAttributes`
- `PutDedicatedIpWarmupAttributes`
- `PutDeliverabilityDashboardOption`
- `PutEmailIdentityConfigurationSetAttributes`
- `PutEmailIdentityDkimAttributes`
- `PutEmailIdentityDkimSigningAttributes`
- `PutEmailIdentityFeedbackAttributes`
- `PutEmailIdentityMailFromAttributes`
- `PutSuppressedDestination`
- `SendBulkEmail`
- `SendCustomVerificationEmail`
- `SendEmail`
- `TagResource`
- `TestRenderEmailTemplate`
- `UntagResource`
- `UpdateConfigurationSetEventDestination`
- `UpdateContact`
- `UpdateContactList`
- `UpdateCustomVerificationEmailTemplate`
- `UpdateEmailIdentityPolicy`
- `UpdateEmailTemplate`
- `UpdateReputationEntityCustomerManagedStatus`
- `UpdateReputationEntityPolicy`

<details><summary>Stubbed APIs (4) &mdash; routed but return an empty/default response</summary>

- `BatchGetMetricData`
- `GetDomainStatisticsReport`
- `GetEmailAddressInsights`
- `ListRecommendations`

</details>
