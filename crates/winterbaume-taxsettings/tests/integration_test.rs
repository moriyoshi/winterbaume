use std::sync::{Arc, Mutex};

use aws_sdk_taxsettings::config::BehaviorVersion;
use aws_sdk_taxsettings::types::{Address, TaxRegistrationEntry};
use winterbaume_core::{MockAws, StatefulService};
use winterbaume_taxsettings::TaxSettingsService;

async fn make_client() -> aws_sdk_taxsettings::Client {
    let mock = MockAws::builder()
        .with_service(TaxSettingsService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_taxsettings::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_taxsettings::Client::new(&config)
}

fn sample_entry() -> TaxRegistrationEntry {
    let address = Address::builder()
        .address_line1("1 Infinite Loop")
        .city("Cupertino")
        .country_code("US")
        .postal_code("95014")
        .state_or_region("CA")
        .build()
        .unwrap();
    TaxRegistrationEntry::builder()
        .registration_id("US-TAX-12345")
        .registration_type("VAT".into())
        .legal_address(address)
        .legal_name("Mock Co")
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_put_get_delete_registration() {
    let client = make_client().await;
    client
        .put_tax_registration()
        .tax_registration_entry(sample_entry())
        .send()
        .await
        .expect("put");

    let got = client.get_tax_registration().send().await.expect("get");
    assert!(got.tax_registration().is_some());

    client
        .delete_tax_registration()
        .send()
        .await
        .expect("delete");

    let after = client.get_tax_registration().send().await.expect("get2");
    assert!(after.tax_registration().is_none());
}

#[tokio::test]
async fn test_inheritance_round_trip() {
    let client = make_client().await;
    let initial = client.get_tax_inheritance().send().await.expect("get");
    assert_eq!(
        initial.heritage_status().map(|s| s.as_str()),
        Some("OptOut")
    );

    client
        .put_tax_inheritance()
        .heritage_status("OptIn".into())
        .send()
        .await
        .expect("put");

    let after = client.get_tax_inheritance().send().await.expect("after");
    assert_eq!(after.heritage_status().map(|s| s.as_str()), Some("OptIn"));
}

#[tokio::test]
async fn test_supplemental_lifecycle() {
    let client = make_client().await;
    let resp = client
        .put_supplemental_tax_registration()
        .tax_registration_entry(
            aws_sdk_taxsettings::types::SupplementalTaxRegistrationEntry::builder()
                .registration_id("CA-TAX-987")
                .registration_type("VAT".into())
                .address(
                    Address::builder()
                        .address_line1("100 Queen St W")
                        .city("Toronto")
                        .country_code("CA")
                        .postal_code("M5H 2N2")
                        .state_or_region("ON")
                        .build()
                        .unwrap(),
                )
                .legal_name("Supplemental Co")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_supplemental");
    let auth_id = resp.authority_id().to_string();
    assert!(auth_id.starts_with("auth-"));

    let list = client
        .list_supplemental_tax_registrations()
        .send()
        .await
        .expect("list_supplemental");
    assert_eq!(list.tax_registrations().len(), 1);

    client
        .delete_supplemental_tax_registration()
        .authority_id(&auth_id)
        .send()
        .await
        .expect("delete_supplemental");
}

#[tokio::test]
async fn test_batch_delete_unknown_returns_errors() {
    let client = make_client().await;
    let resp = client
        .batch_delete_tax_registration()
        .account_ids("123")
        .account_ids("456")
        .send()
        .await
        .expect("batch_delete");
    assert_eq!(resp.errors().len(), 2);
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = TaxSettingsService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });
    svc.restore("123456789012", "us-east-1", Default::default())
        .await
        .expect("restore");
    assert_eq!(events.lock().unwrap().len(), 1);
}
