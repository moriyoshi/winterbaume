//! Keyspaces clients now negotiate RPC-v2 CBOR as well as AWS JSON 1.0.

use ciborium::value::Value as CborValue;
use serde_json::{Value, json};
use winterbaume_core::{MockRequest, MockResponse, MockService};
use winterbaume_keyspaces::KeyspacesService;

fn request(operation: &str, body: &impl serde::Serialize) -> MockRequest {
    let mut bytes = Vec::new();
    ciborium::into_writer(body, &mut bytes).unwrap();
    let mut headers = http::HeaderMap::new();
    headers.insert("smithy-protocol", "rpc-v2-cbor".parse().unwrap());
    headers.insert(
        http::header::CONTENT_TYPE,
        "application/cbor".parse().unwrap(),
    );
    MockRequest {
        method: "POST".into(),
        uri: format!(
            "https://cassandra.us-east-1.amazonaws.com/service/KeyspacesService/operation/{operation}"
        ),
        headers,
        body: bytes.into(),
    }
}

fn json_body(response: &MockResponse, status: u16) -> Value {
    assert_eq!(response.status, status);
    assert_eq!(response.headers["smithy-protocol"], "rpc-v2-cbor");
    assert_eq!(
        response.headers[http::header::CONTENT_TYPE],
        "application/cbor"
    );
    ciborium::from_reader(response.body.as_ref()).unwrap()
}

fn field<'a>(value: &'a CborValue, name: &str) -> &'a CborValue {
    &value
        .as_map()
        .unwrap()
        .iter()
        .find(|(key, _)| key.as_text() == Some(name))
        .unwrap()
        .1
}

#[tokio::test]
async fn cbor_keyspace_lifecycle_shares_json_state_and_errors() {
    let service = KeyspacesService::new();
    let create =
        json!({"keyspaceName": "protocol_ks", "tags": [{"key": "source", "value": "cbor"}]});
    let out = service.handle(request("CreateKeyspace", &create)).await;
    let arn = json_body(&out, 200)["resourceArn"]
        .as_str()
        .unwrap()
        .to_owned();
    let duplicate = service.handle(request("CreateKeyspace", &create)).await;
    assert_eq!(
        json_body(&duplicate, 409)["__type"],
        "com.amazonaws.keyspaces#ConflictException"
    );

    let mut json_request = request("GetKeyspace", &json!({}));
    json_request.uri = "https://cassandra.us-east-1.amazonaws.com/".into();
    json_request.headers.remove("smithy-protocol");
    json_request.headers.insert(
        "x-amz-target",
        "KeyspacesService.GetKeyspace".parse().unwrap(),
    );
    json_request.headers.insert(
        http::header::CONTENT_TYPE,
        "application/x-amz-json-1.0".parse().unwrap(),
    );
    json_request.body = serde_json::to_vec(&json!({"keyspaceName": "protocol_ks"}))
        .unwrap()
        .into();
    let out = service.handle(json_request).await;
    assert_eq!(out.status, 200);
    assert!(!out.headers.contains_key("smithy-protocol"));
    let body: Value = serde_json::from_slice(&out.body).unwrap();
    assert_eq!(body["resourceArn"], arn);
    assert_eq!(body["keyspaceName"], "protocol_ks");

    let tags = service
        .handle(request("ListTagsForResource", &json!({"resourceArn": arn})))
        .await;
    assert_eq!(
        json_body(&tags, 200)["tags"],
        json!([{"key": "source", "value": "cbor"}])
    );
    // Empty structures and a runtime path prefix must both be supported.
    let mut list = request("ListKeyspaces", &json!({}));
    list.uri = list.uri.replace("/service/", "/v1/service/");
    let out = service.handle(list).await;
    assert_eq!(
        json_body(&out, 200)["keyspaces"][0]["keyspaceName"],
        "protocol_ks"
    );
    let out = service
        .handle(request(
            "DeleteKeyspace",
            &json!({"keyspaceName": "protocol_ks"}),
        ))
        .await;
    json_body(&out, 200);
    let out = service
        .handle(request(
            "GetKeyspace",
            &json!({"keyspaceName": "protocol_ks"}),
        ))
        .await;
    assert_eq!(
        json_body(&out, 404)["__type"],
        "com.amazonaws.keyspaces#ResourceNotFoundException"
    );
}

#[tokio::test]
async fn cbor_tables_use_tagged_timestamps_and_accept_restore_timestamp() {
    let service = KeyspacesService::new();
    let out = service
        .handle(request(
            "CreateKeyspace",
            &json!({"keyspaceName": "tables_ks"}),
        ))
        .await;
    json_body(&out, 200);
    let create = json!({
        "keyspaceName": "tables_ks", "tableName": "source_table",
        "schemaDefinition": {"allColumns": [{"name": "id", "type": "text"}], "partitionKeys": [{"name": "id"}]},
        "capacitySpecification": {"throughputMode": "PAY_PER_REQUEST"}
    });
    let out = service.handle(request("CreateTable", &create)).await;
    json_body(&out, 200);
    let out = service
        .handle(request(
            "GetTable",
            &json!({"keyspaceName": "tables_ks", "tableName": "source_table"}),
        ))
        .await;
    assert_eq!(out.status, 200);
    let body: CborValue = ciborium::from_reader(out.body.as_ref()).unwrap();
    assert_eq!(field(&body, "tableName").as_text(), Some("source_table"));
    assert_eq!(
        field(field(&body, "schemaDefinition"), "partitionKeys")
            .as_array()
            .unwrap()
            .len(),
        1
    );
    let timestamp = field(&body, "creationTimestamp");
    assert!(
        matches!(timestamp, CborValue::Tag(1, value) if matches!(value.as_ref(), CborValue::Integer(_) | CborValue::Float(_)))
    );

    let restore = CborValue::Map(vec![
        ("sourceKeyspaceName".into(), "tables_ks".into()),
        ("sourceTableName".into(), "source_table".into()),
        ("targetKeyspaceName".into(), "tables_ks".into()),
        ("targetTableName".into(), "restored_table".into()),
        ("restoreTimestamp".into(), timestamp.clone()),
    ]);
    let out = service.handle(request("RestoreTable", &restore)).await;
    assert!(
        json_body(&out, 200)["restoredTableARN"]
            .as_str()
            .unwrap()
            .contains("restored_table")
    );
}

#[tokio::test]
async fn cbor_rejects_malformed_requests_with_cbor_errors() {
    let service = KeyspacesService::new();
    let mut cases = Vec::new();
    let mut invalid = request("ListKeyspaces", &json!({}));
    invalid.body = bytes::Bytes::from_static(&[0xff]);
    cases.push(invalid);
    let mut trailing = request("ListKeyspaces", &json!({}));
    trailing.body = bytes::Bytes::from_static(&[0xa0, 0xa0]);
    cases.push(trailing);
    cases.push(request("ListKeyspaces", &json!([1, 2])));
    for target in ["x-amz-target", "x-amzn-target"] {
        let mut ambiguous = request("ListKeyspaces", &json!({}));
        ambiguous
            .headers
            .insert(target, "KeyspacesService.ListKeyspaces".parse().unwrap());
        cases.push(ambiguous);
    }
    let mut method = request("ListKeyspaces", &json!({}));
    method.method = "GET".into();
    cases.push(method);
    let mut path = request("ListKeyspaces", &json!({}));
    path.uri = path.uri.replace("KeyspacesService", "OtherService");
    cases.push(path);
    for request in cases {
        let out = service.handle(request).await;
        assert_eq!(
            json_body(&out, 400)["__type"],
            "com.amazonaws.keyspaces#ValidationException"
        );
    }
    let out = service.handle(request("CreateKeyspace", &json!({}))).await;
    assert_eq!(
        json_body(&out, 400)["__type"],
        "com.amazonaws.keyspaces#ValidationException"
    );
}
