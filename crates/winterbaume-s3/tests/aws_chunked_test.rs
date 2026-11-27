//! `aws-chunked` request bodies ( issue #84 ): streaming uploads store the
//! decoded data, not the chunk framing, and trailing checksums are verified.
//!
//! The raw requests mirror what current SDKs send: aws-sdk-cpp ( pyarrow )
//! uses `STREAMING-UNSIGNED-PAYLOAD-TRAILER` with a CRC64NVME trailer, and the
//! signed variants carry `;chunk-signature=` extensions.

use std::sync::Arc;

use aws_sdk_s3::config::BehaviorVersion;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::ChecksumAlgorithm;
use bytes::Bytes;
use winterbaume_core::{MockAws, MockRequest, MockResponse, MockService};
use winterbaume_s3::S3Service;

const ENDPOINT: &str = "http://s3.us-east-1.amazonaws.com";
const SIG: &str = "ad80c730a21e5b8d04586a2213dd63b9a0e99e0e2307b0ade35a65485a288648";

async fn send(
    service: &Arc<S3Service>,
    method: &str,
    path: &str,
    headers: &[(&str, &str)],
    body: impl Into<Bytes>,
) -> MockResponse {
    let mut map = http::HeaderMap::new();
    map.insert(
        "authorization",
        "AWS4-HMAC-SHA256 Credential=test/20260101/us-east-1/s3/aws4_request, SignedHeaders=host;x-amz-date, Signature=0000"
            .parse()
            .unwrap(),
    );
    map.insert("x-amz-date", "20260101T000000Z".parse().unwrap());
    for (name, value) in headers {
        map.append(
            http::header::HeaderName::from_bytes(name.as_bytes()).unwrap(),
            value.parse().unwrap(),
        );
    }
    service
        .handle(MockRequest {
            method: method.to_string(),
            uri: format!("{ENDPOINT}{path}"),
            headers: map,
            body: body.into(),
        })
        .await
}

async fn service_with_bucket(bucket: &str) -> Arc<S3Service> {
    let service = Arc::new(S3Service::new());
    let created = send(&service, "PUT", &format!("/{bucket}"), &[], Bytes::new()).await;
    assert_eq!(created.status, 200);
    service
}

async fn get_body(service: &Arc<S3Service>, path: &str) -> Bytes {
    let response = send(service, "GET", path, &[], Bytes::new()).await;
    assert_eq!(response.status, 200, "{:?}", response.body);
    response.body
}

fn error_code(response: &MockResponse) -> String {
    let body = String::from_utf8_lossy(&response.body);
    body.split("<Code>")
        .nth(1)
        .and_then(|rest| rest.split("</Code>").next())
        .unwrap_or_default()
        .to_string()
}

fn chunked(chunks: &[&[u8]], signed: bool, trailers: &[(&str, &str)]) -> Vec<u8> {
    let ext = if signed {
        format!(";chunk-signature={SIG}")
    } else {
        String::new()
    };
    let mut body = Vec::new();
    for chunk in chunks {
        body.extend_from_slice(format!("{:x}{ext}\r\n", chunk.len()).as_bytes());
        body.extend_from_slice(chunk);
        body.extend_from_slice(b"\r\n");
    }
    body.extend_from_slice(format!("0{ext}\r\n").as_bytes());
    for (name, value) in trailers {
        body.extend_from_slice(format!("{name}:{value}\r\n").as_bytes());
    }
    body.extend_from_slice(b"\r\n");
    body
}

/// The reproduction from the issue: an unsigned body with a CRC32 trailer.
#[tokio::test]
async fn put_object_unsigned_trailer_stores_the_decoded_data() {
    let service = service_with_bucket("bkt").await;
    let response = send(
        &service,
        "PUT",
        "/bkt/c",
        &[
            ("x-amz-content-sha256", "STREAMING-UNSIGNED-PAYLOAD-TRAILER"),
            ("content-encoding", "aws-chunked"),
            ("x-amz-decoded-content-length", "5"),
            ("x-amz-trailer", "x-amz-checksum-crc32"),
        ],
        b"5\r\nhello\r\n0\r\nx-amz-checksum-crc32:NhCmhg==\r\n\r\n".to_vec(),
    )
    .await;
    assert_eq!(response.status, 200, "{:?}", response.body);
    assert_eq!(
        get_body(&service, "/bkt/c").await,
        Bytes::from_static(b"hello")
    );

    let head = send(&service, "HEAD", "/bkt/c", &[], Bytes::new()).await;
    assert_eq!(head.headers.get("content-length").unwrap(), "5");
}

/// pyarrow's shape: one 5000-byte chunk and a CRC64NVME trailer. Before the
/// fix the object was stored as 5052 bytes.
#[tokio::test]
async fn put_object_large_chunk_with_crc64nvme_trailer() {
    let service = service_with_bucket("bkt").await;
    let data = vec![b'A'; 5000];
    let mut checksum = aws_smithy_checksums::ChecksumAlgorithm::Crc64Nvme.into_impl();
    checksum.update(&data);
    let crc = checksum.header_value();
    let body = chunked(
        &[&data],
        false,
        &[("x-amz-checksum-crc64nvme", crc.to_str().unwrap())],
    );
    assert_eq!(&body[..6], b"1388\r\n");
    let response = send(
        &service,
        "PUT",
        "/bkt/x",
        &[
            ("x-amz-content-sha256", "STREAMING-UNSIGNED-PAYLOAD-TRAILER"),
            ("content-encoding", "aws-chunked"),
            ("x-amz-decoded-content-length", "5000"),
            ("x-amz-trailer", "x-amz-checksum-crc64nvme"),
        ],
        body,
    )
    .await;
    assert_eq!(response.status, 200, "{:?}", response.body);
    assert_eq!(get_body(&service, "/bkt/x").await, Bytes::from(data));
}

#[tokio::test]
async fn put_object_signed_payload_in_several_chunks() {
    let service = service_with_bucket("bkt").await;
    let response = send(
        &service,
        "PUT",
        "/bkt/signed",
        &[
            ("x-amz-content-sha256", "STREAMING-AWS4-HMAC-SHA256-PAYLOAD"),
            ("content-encoding", "aws-chunked"),
            ("x-amz-decoded-content-length", "11"),
        ],
        chunked(&[b"hello", b" ", b"world"], true, &[]),
    )
    .await;
    assert_eq!(response.status, 200, "{:?}", response.body);
    assert_eq!(
        get_body(&service, "/bkt/signed").await,
        Bytes::from_static(b"hello world")
    );
}

#[tokio::test]
async fn put_object_signed_payload_with_signed_trailer() {
    let service = service_with_bucket("bkt").await;
    let response = send(
        &service,
        "PUT",
        "/bkt/signed-trailer",
        &[
            (
                "x-amz-content-sha256",
                "STREAMING-AWS4-HMAC-SHA256-PAYLOAD-TRAILER",
            ),
            ("content-encoding", "aws-chunked"),
            ("x-amz-decoded-content-length", "5"),
            ("x-amz-trailer", "x-amz-checksum-crc32"),
        ],
        chunked(
            &[b"hello"],
            true,
            &[
                ("x-amz-checksum-crc32", "NhCmhg=="),
                ("x-amz-trailer-signature", SIG),
            ],
        ),
    )
    .await;
    assert_eq!(response.status, 200, "{:?}", response.body);
    assert_eq!(
        get_body(&service, "/bkt/signed-trailer").await,
        Bytes::from_static(b"hello")
    );
}

/// `aws-chunked` listed with another encoding is still decoded.
#[tokio::test]
async fn put_object_content_encoding_list() {
    let service = service_with_bucket("bkt").await;
    let response = send(
        &service,
        "PUT",
        "/bkt/listed",
        &[("content-encoding", "aws-chunked,gzip")],
        chunked(&[b"hello"], false, &[]),
    )
    .await;
    assert_eq!(response.status, 200, "{:?}", response.body);
    assert_eq!(
        get_body(&service, "/bkt/listed").await,
        Bytes::from_static(b"hello")
    );
}

#[tokio::test]
async fn upload_part_decodes_aws_chunked_parts() {
    let service = service_with_bucket("bkt").await;
    let created = send(&service, "POST", "/bkt/mpu?uploads", &[], Bytes::new()).await;
    assert_eq!(created.status, 200);
    let xml = String::from_utf8_lossy(&created.body).to_string();
    let upload_id = xml
        .split("<UploadId>")
        .nth(1)
        .and_then(|rest| rest.split("</UploadId>").next())
        .expect("UploadId")
        .to_string();

    let part = send(
        &service,
        "PUT",
        &format!("/bkt/mpu?partNumber=1&uploadId={upload_id}"),
        &[
            ("x-amz-content-sha256", "STREAMING-UNSIGNED-PAYLOAD-TRAILER"),
            ("content-encoding", "aws-chunked"),
            ("x-amz-decoded-content-length", "5"),
            ("x-amz-trailer", "x-amz-checksum-crc32"),
        ],
        chunked(&[b"hello"], false, &[("x-amz-checksum-crc32", "NhCmhg==")]),
    )
    .await;
    assert_eq!(part.status, 200, "{:?}", part.body);
    let etag = part
        .headers
        .get("etag")
        .expect("part ETag")
        .to_str()
        .unwrap()
        .to_string();
    // The part ETag is the MD5 of the decoded data.
    assert_eq!(etag, "\"5d41402abc4b2a76b9719d911017c592\"");

    let complete = format!(
        "<CompleteMultipartUpload><Part><PartNumber>1</PartNumber><ETag>{etag}</ETag></Part></CompleteMultipartUpload>"
    );
    let completed = send(
        &service,
        "POST",
        &format!("/bkt/mpu?uploadId={upload_id}"),
        &[],
        complete,
    )
    .await;
    assert_eq!(completed.status, 200, "{:?}", completed.body);
    assert_eq!(
        get_body(&service, "/bkt/mpu").await,
        Bytes::from_static(b"hello")
    );
}

#[tokio::test]
async fn bad_trailing_checksum_is_bad_digest() {
    let service = service_with_bucket("bkt").await;
    let response = send(
        &service,
        "PUT",
        "/bkt/bad",
        &[
            ("x-amz-content-sha256", "STREAMING-UNSIGNED-PAYLOAD-TRAILER"),
            ("content-encoding", "aws-chunked"),
            ("x-amz-decoded-content-length", "5"),
        ],
        chunked(&[b"hellp"], false, &[("x-amz-checksum-crc32", "NhCmhg==")]),
    )
    .await;
    assert_eq!(response.status, 400);
    assert_eq!(error_code(&response), "BadDigest");
    assert!(
        String::from_utf8_lossy(&response.body).contains("The CRC32 you specified"),
        "{:?}",
        response.body
    );
    // Nothing was stored.
    let get = send(&service, "GET", "/bkt/bad", &[], Bytes::new()).await;
    assert_eq!(get.status, 404);
}

#[tokio::test]
async fn decoded_length_mismatch_is_incomplete_body() {
    let service = service_with_bucket("bkt").await;
    let response = send(
        &service,
        "PUT",
        "/bkt/short",
        &[
            ("content-encoding", "aws-chunked"),
            ("x-amz-decoded-content-length", "6"),
        ],
        chunked(&[b"hello"], false, &[]),
    )
    .await;
    assert_eq!(response.status, 400);
    assert_eq!(error_code(&response), "IncompleteBody");
}

#[tokio::test]
async fn truncated_body_is_incomplete_body() {
    let service = service_with_bucket("bkt").await;
    let response = send(
        &service,
        "PUT",
        "/bkt/cut",
        &[("content-encoding", "aws-chunked")],
        b"a\r\nhello".to_vec(),
    )
    .await;
    assert_eq!(response.status, 400);
    assert_eq!(error_code(&response), "IncompleteBody");
}

#[tokio::test]
async fn malformed_chunk_size_is_invalid_request() {
    let service = service_with_bucket("bkt").await;
    let response = send(
        &service,
        "PUT",
        "/bkt/bad-size",
        &[("content-encoding", "aws-chunked")],
        b"zz\r\nhello\r\n0\r\n\r\n".to_vec(),
    )
    .await;
    assert_eq!(response.status, 400);
    assert_eq!(error_code(&response), "InvalidRequest");
}

/// A body that merely looks framed is stored as sent when the request does
/// not say it is `aws-chunked`.
#[tokio::test]
async fn plain_body_is_stored_unchanged() {
    let service = service_with_bucket("bkt").await;
    let raw = b"5\r\nhello\r\n0\r\n\r\n".to_vec();
    let response = send(&service, "PUT", "/bkt/plain", &[], raw.clone()).await;
    assert_eq!(response.status, 200);
    assert_eq!(get_body(&service, "/bkt/plain").await, Bytes::from(raw));
}

/// The Rust SDK with a requested checksum, through the in-process client.
/// With an in-memory body it sends the checksum as a header rather than
/// `aws-chunked`; this guards that path against the decoder.
///
/// A streaming body cannot be used here: `MockAwsClient` copies only bodies
/// whose bytes are already available, so a `ByteStream::from_path` upload
/// reaches the service empty ( `mock-client-streaming-bodies` in `TODO.md` ).
#[tokio::test]
async fn sdk_upload_with_checksum_round_trips() {
    let mock = MockAws::builder().with_service(S3Service::new()).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3::config::Region::new("us-east-1"))
        .load()
        .await;
    let client = aws_sdk_s3::Client::new(&config);
    client.create_bucket().bucket("bkt").send().await.unwrap();

    let data = vec![b'A'; 70_000];
    for algorithm in [ChecksumAlgorithm::Crc32, ChecksumAlgorithm::Crc64Nvme] {
        let key = format!("sdk-{}", algorithm.as_str());
        client
            .put_object()
            .bucket("bkt")
            .key(&key)
            .checksum_algorithm(algorithm.clone())
            .body(ByteStream::from(data.clone()))
            .send()
            .await
            .unwrap();
        let got = client
            .get_object()
            .bucket("bkt")
            .key(&key)
            .send()
            .await
            .unwrap()
            .body
            .collect()
            .await
            .unwrap()
            .into_bytes();
        assert_eq!(got, Bytes::from(data.clone()), "{key}");
    }
}
