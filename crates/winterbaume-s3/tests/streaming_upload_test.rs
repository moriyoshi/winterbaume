//! Streaming request bodies through the in-process client.
//!
//! `MockAwsClient` used to copy only bodies already in memory, so an upload
//! from `ByteStream::from_path` reached S3 empty and was stored as a
//! zero-byte object without any error.

use aws_sdk_s3::config::{BehaviorVersion, RequestChecksumCalculation};
use aws_sdk_s3::primitives::ByteStream;
use bytes::Bytes;
use winterbaume_core::MockAws;
use winterbaume_s3::S3Service;

#[tokio::test]
async fn put_object_from_a_file_stores_its_content() {
    let mock = MockAws::builder().with_service(S3Service::new()).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3::config::Region::new("us-east-1"))
        .load()
        .await;
    // Without a requested checksum the SDK sends the file unframed; with one
    // it uses `aws-chunked`, which S3 decodes separately ( issue #84 ).
    let s3_config = aws_sdk_s3::config::Builder::from(&config)
        .request_checksum_calculation(RequestChecksumCalculation::WhenRequired)
        .build();
    let client = aws_sdk_s3::Client::from_conf(s3_config);
    client.create_bucket().bucket("bkt").send().await.unwrap();

    let dir = std::env::temp_dir().join(format!("wb-s3-stream-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("payload");
    let data: Vec<u8> = (0..300_000u32).map(|i| (i % 251) as u8).collect();
    std::fs::write(&path, &data).unwrap();

    client
        .put_object()
        .bucket("bkt")
        .key("from-file")
        .body(ByteStream::from_path(&path).await.unwrap())
        .send()
        .await
        .unwrap();
    std::fs::remove_dir_all(&dir).unwrap();

    let head = client
        .head_object()
        .bucket("bkt")
        .key("from-file")
        .send()
        .await
        .unwrap();
    assert_eq!(head.content_length(), Some(data.len() as i64));
    let got = client
        .get_object()
        .bucket("bkt")
        .key("from-file")
        .send()
        .await
        .unwrap()
        .body
        .collect()
        .await
        .unwrap()
        .into_bytes();
    assert_eq!(got, Bytes::from(data));
}
