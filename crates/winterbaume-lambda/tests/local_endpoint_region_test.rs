//! Requests sent to a local address ( `winterbaume-server` at
//! `http://127.0.0.1:<port>` ) carry their region only in the SigV4
//! credential scope.  Lambda must use it, not fall back to `us-east-1`.

use winterbaume_core::{MockRequest, MockService};
use winterbaume_lambda::LambdaService;

fn request(method: &str, path: &str, body: &str) -> MockRequest {
    let mut headers = http::HeaderMap::new();
    headers.insert(
        "authorization",
        "AWS4-HMAC-SHA256 Credential=AKID/20260101/eu-west-1/lambda/aws4_request, SignedHeaders=host, Signature=0"
            .parse()
            .unwrap(),
    );
    headers.insert("content-type", "application/json".parse().unwrap());
    MockRequest {
        method: method.to_string(),
        uri: format!("https://127.0.0.1:5555{path}"),
        headers,
        body: body.to_string().into(),
    }
}

#[tokio::test]
async fn functions_created_through_a_local_address_live_in_the_signed_region() {
    let service = LambdaService::new();
    let created = service
        .handle(request(
            "POST",
            "/2015-03-31/functions",
            r#"{"FunctionName":"f","Runtime":"nodejs22.x","Handler":"index.handler","Role":"arn:aws:iam::123456789012:role/r","Code":{"ZipFile":"UEsFBgAAAAAAAAAAAAAAAAAAAAAAAA=="}}"#,
        ))
        .await;
    assert_eq!(
        created.status,
        201,
        "{}",
        String::from_utf8_lossy(&created.body)
    );
    let body: serde_json::Value = serde_json::from_slice(&created.body).unwrap();
    assert_eq!(
        body["FunctionArn"],
        "arn:aws:lambda:eu-west-1:123456789012:function:f"
    );
    assert_eq!(service.scopes_with_state()[0].1, "eu-west-1");

    let fetched = service
        .handle(request("GET", "/2015-03-31/functions/f", ""))
        .await;
    assert_eq!(fetched.status, 200);
}
