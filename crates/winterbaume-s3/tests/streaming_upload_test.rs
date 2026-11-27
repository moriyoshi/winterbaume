//! Streaming request bodies through the in-process client.
//!
//! `MockAwsClient` used to copy only bodies already in memory, so an upload
//! from `ByteStream::from_path` reached S3 empty and was stored as a
//! zero-byte object without any error.

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use aws_sdk_s3::config::{BehaviorVersion, RequestChecksumCalculation};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::ChecksumAlgorithm;
use bytes::Bytes;
use winterbaume_core::{MockAws, MockRequest, MockResponse, MockService};
use winterbaume_s3::S3Service;

/// Records the `Content-Encoding` of every request before passing it on.
struct Recording {
    inner: S3Service,
    encodings: Mutex<Vec<String>>,
}

impl MockService for Recording {
    fn service_name(&self) -> &str {
        self.inner.service_name()
    }

    fn url_patterns(&self) -> Vec<&str> {
        self.inner.url_patterns()
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        let encoding = request
            .headers
            .get("content-encoding")
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default()
            .to_owned();
        self.encodings.lock().unwrap().push(encoding);
        self.inner.handle(request)
    }
}

async fn client(
    service: impl MockService,
    checksums: RequestChecksumCalculation,
) -> aws_sdk_s3::Client {
    let mock = MockAws::builder().with_service(service).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3::config::Region::new("us-east-1"))
        .load()
        .await;
    let s3_config = aws_sdk_s3::config::Builder::from(&config)
        .request_checksum_calculation(checksums)
        .build();
    let client = aws_sdk_s3::Client::from_conf(s3_config);
    client.create_bucket().bucket("bkt").send().await.unwrap();
    client
}

/// Uploads `data` from a file under `key`, then checks the stored object.
async fn upload_from_file_and_read_back(
    client: &aws_sdk_s3::Client,
    key: &str,
    data: &[u8],
    checksum: Option<ChecksumAlgorithm>,
) {
    let dir = std::env::temp_dir().join(format!("wb-s3-stream-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("payload");
    std::fs::write(&path, data).unwrap();

    client
        .put_object()
        .bucket("bkt")
        .key(key)
        .set_checksum_algorithm(checksum)
        .body(ByteStream::from_path(&path).await.unwrap())
        .send()
        .await
        .unwrap();
    std::fs::remove_dir_all(&dir).unwrap();

    let head = client
        .head_object()
        .bucket("bkt")
        .key(key)
        .send()
        .await
        .unwrap();
    assert_eq!(head.content_length(), Some(data.len() as i64), "{key}");
    let got = client
        .get_object()
        .bucket("bkt")
        .key(key)
        .send()
        .await
        .unwrap()
        .body
        .collect()
        .await
        .unwrap()
        .into_bytes();
    assert_eq!(got, Bytes::copy_from_slice(data), "{key}");
}

fn payload() -> Vec<u8> {
    (0..300_000u32).map(|i| (i % 251) as u8).collect()
}

#[tokio::test]
async fn put_object_from_a_file_stores_its_content() {
    // Without a requested checksum the SDK sends the file unframed.
    let client = client(S3Service::new(), RequestChecksumCalculation::WhenRequired).await;
    upload_from_file_and_read_back(&client, "from-file", &payload(), None).await;
}

/// With a checksum, the SDK streams a file as `aws-chunked` with the checksum
/// as a trailer, which S3 decodes and verifies ( issue #84 ).
#[tokio::test]
async fn put_object_from_a_file_with_a_checksum_is_sent_aws_chunked() {
    let service = Arc::new(Recording {
        inner: S3Service::new(),
        encodings: Mutex::new(Vec::new()),
    });
    let client = client(service.clone(), RequestChecksumCalculation::WhenSupported).await;
    let data = payload();
    for algorithm in [ChecksumAlgorithm::Crc32, ChecksumAlgorithm::Crc64Nvme] {
        let key = format!("from-file-{}", algorithm.as_str());
        upload_from_file_and_read_back(&client, &key, &data, Some(algorithm)).await;
    }

    let encodings = service.encodings.lock().unwrap();
    let chunked = encodings
        .iter()
        .filter(|e| e.contains("aws-chunked"))
        .count();
    assert_eq!(chunked, 2, "{encodings:?}");
}
