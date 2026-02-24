use std::sync::{Arc, Mutex};

use aws_sdk_connectparticipant::config::BehaviorVersion;
use winterbaume_connectparticipant::ConnectParticipantService;
use winterbaume_core::{MockAws, StatefulService};

async fn make_client() -> aws_sdk_connectparticipant::Client {
    let mock = MockAws::builder()
        .with_service(ConnectParticipantService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_connectparticipant::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_connectparticipant::Client::new(&config)
}

async fn open_connection(client: &aws_sdk_connectparticipant::Client) -> String {
    let resp = client
        .create_participant_connection()
        .participant_token("token-abc")
        .r#type("WEBSOCKET".into())
        .r#type("CONNECTION_CREDENTIALS".into())
        .send()
        .await
        .expect("create connection");
    resp.connection_credentials()
        .expect("creds")
        .connection_token()
        .expect("token")
        .to_string()
}

#[tokio::test]
async fn test_create_then_send_message() {
    let client = make_client().await;
    let token = open_connection(&client).await;

    let msg = client
        .send_message()
        .connection_token(&token)
        .content_type("text/plain")
        .content("hello")
        .send()
        .await
        .expect("send message");
    assert!(msg.id().unwrap_or_default().starts_with("msg-"));

    let transcript = client
        .get_transcript()
        .connection_token(&token)
        .send()
        .await
        .expect("transcript");
    assert_eq!(transcript.transcript().len(), 1);
    assert_eq!(transcript.transcript()[0].content(), Some("hello"));
}

#[tokio::test]
async fn test_send_event_records_typing_event() {
    let client = make_client().await;
    let token = open_connection(&client).await;

    client
        .send_event()
        .connection_token(&token)
        .content_type("application/vnd.amazonaws.connect.event.typing")
        .send()
        .await
        .expect("send event");
    let transcript = client
        .get_transcript()
        .connection_token(&token)
        .send()
        .await
        .expect("transcript");
    assert_eq!(transcript.transcript().len(), 1);
}

#[tokio::test]
async fn test_attachment_lifecycle() {
    let client = make_client().await;
    let token = open_connection(&client).await;

    let upload = client
        .start_attachment_upload()
        .connection_token(&token)
        .content_type("image/png")
        .attachment_size_in_bytes(123)
        .attachment_name("avatar.png")
        .client_token("client-1")
        .send()
        .await
        .expect("start upload");
    let attachment_id = upload.attachment_id().expect("id").to_string();
    assert!(attachment_id.starts_with("att-"));

    client
        .complete_attachment_upload()
        .connection_token(&token)
        .attachment_ids(&attachment_id)
        .client_token("client-1")
        .send()
        .await
        .expect("complete");
}

#[tokio::test]
async fn test_disconnect_invalidates_token() {
    let client = make_client().await;
    let token = open_connection(&client).await;

    client
        .disconnect_participant()
        .connection_token(&token)
        .send()
        .await
        .expect("disconnect");

    let err = client
        .send_message()
        .connection_token(&token)
        .content_type("text/plain")
        .content("after disconnect")
        .send()
        .await
        .expect_err("rejected");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = ConnectParticipantService::new();
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
