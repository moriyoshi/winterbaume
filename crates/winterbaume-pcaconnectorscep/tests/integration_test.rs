use std::sync::{Arc, Mutex};

use aws_sdk_pcaconnectorscep::config::BehaviorVersion;
use winterbaume_core::{MockAws, StatefulService};
use winterbaume_pcaconnectorscep::PcaConnectorScepService;

async fn make_client() -> aws_sdk_pcaconnectorscep::Client {
    let mock = MockAws::builder()
        .with_service(PcaConnectorScepService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_pcaconnectorscep::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_pcaconnectorscep::Client::new(&config)
}

#[tokio::test]
async fn test_connector_lifecycle() {
    let client = make_client().await;
    let create = client
        .create_connector()
        .certificate_authority_arn("arn:aws:acm-pca:us-east-1:123:certificate-authority/abc")
        .send()
        .await
        .expect("create_connector");
    let arn = create.connector_arn().expect("arn").to_string();

    let get = client
        .get_connector()
        .connector_arn(&arn)
        .send()
        .await
        .expect("get_connector");
    let connector = get.connector().expect("connector");
    assert_eq!(connector.status().map(|s| s.as_str()), Some("ACTIVE"));

    let list = client
        .list_connectors()
        .send()
        .await
        .expect("list_connectors");
    assert_eq!(list.connectors().len(), 1);

    client
        .delete_connector()
        .connector_arn(&arn)
        .send()
        .await
        .expect("delete_connector");
}

#[tokio::test]
async fn test_challenge_lifecycle() {
    let client = make_client().await;
    let connector = client
        .create_connector()
        .certificate_authority_arn("arn:aws:acm-pca:us-east-1:123:certificate-authority/abc")
        .send()
        .await
        .expect("create_connector");
    let connector_arn = connector.connector_arn().expect("arn").to_string();

    let challenge = client
        .create_challenge()
        .connector_arn(&connector_arn)
        .send()
        .await
        .expect("create_challenge");
    let ch = challenge.challenge().expect("challenge");
    let challenge_arn = ch.arn().expect("arn").to_string();
    assert!(ch.password().unwrap_or_default().starts_with("scep-"));

    let metadata = client
        .get_challenge_metadata()
        .challenge_arn(&challenge_arn)
        .send()
        .await
        .expect("get_challenge_metadata");
    assert!(metadata.challenge_metadata().is_some());

    let pwd = client
        .get_challenge_password()
        .challenge_arn(&challenge_arn)
        .send()
        .await
        .expect("get_challenge_password");
    assert!(pwd.password().unwrap_or_default().starts_with("scep-"));

    let list = client
        .list_challenge_metadata()
        .connector_arn(&connector_arn)
        .send()
        .await
        .expect("list_challenge_metadata");
    assert_eq!(list.challenges().len(), 1);

    client
        .delete_challenge()
        .challenge_arn(&challenge_arn)
        .send()
        .await
        .expect("delete_challenge");
}

#[tokio::test]
async fn test_create_challenge_unknown_connector() {
    let client = make_client().await;
    let err = client
        .create_challenge()
        .connector_arn("arn:aws:pca-connector-scep:us-east-1:123:connector/missing")
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_get_connector_not_found() {
    let client = make_client().await;
    let err = client
        .get_connector()
        .connector_arn("arn:aws:pca-connector-scep:us-east-1:123:connector/missing")
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = PcaConnectorScepService::new();
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
