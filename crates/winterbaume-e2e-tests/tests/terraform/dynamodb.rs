use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use bytes::Bytes;
use winterbaume_core::service::{MockRequest, MockResponse, MockService};

use crate::harness::*;

// ---------------------------------------------------------------------------
// WarmThroughput-patching wrapper for DynamoDB service
// ---------------------------------------------------------------------------

/// Wraps the real DynamoDB service and injects `WarmThroughput` into
/// DescribeTable / CreateTable / UpdateTable responses.
///
/// terraform-provider-aws v6.13.0+ polls `WarmThroughput.Status` in the
/// DescribeTable response and enters an infinite retry loop when the field
/// is absent. Real AWS always returns this field.
struct DynamoDbWarmThroughputPatch {
    inner: Arc<dyn MockService>,
}

impl DynamoDbWarmThroughputPatch {
    fn new(inner: Arc<dyn MockService>) -> Self {
        Self { inner }
    }
}

impl MockService for DynamoDbWarmThroughputPatch {
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
        Box::pin(async move {
            let response = self.inner.handle(request).await;

            // Only patch 200 JSON responses that contain a TableDescription
            if response.status != 200 {
                return response;
            }

            let body_str = match std::str::from_utf8(&response.body) {
                Ok(s) => s,
                Err(_) => return response,
            };

            // Check if this is a response containing TableDescription
            // (from DescribeTable, CreateTable, UpdateTable, etc.)
            if !body_str.contains("\"TableStatus\"") {
                return response;
            }

            // Inject WarmThroughput into the TableDescription if not already present
            if body_str.contains("\"WarmThroughput\"") {
                return response;
            }

            // Insert WarmThroughput right after TableStatus
            let patched = body_str.replace(
                "\"TableStatus\":\"ACTIVE\"",
                "\"TableStatus\":\"ACTIVE\",\"WarmThroughput\":{\"ReadUnitsPerSecond\":12000,\"Status\":\"ACTIVE\",\"WriteUnitsPerSecond\":4000}",
            ).replace(
                "\"TableStatus\":\"CREATING\"",
                "\"TableStatus\":\"CREATING\",\"WarmThroughput\":{\"ReadUnitsPerSecond\":12000,\"Status\":\"ACTIVE\",\"WriteUnitsPerSecond\":4000}",
            ).replace(
                "\"TableStatus\":\"DELETING\"",
                "\"TableStatus\":\"DELETING\",\"WarmThroughput\":{\"ReadUnitsPerSecond\":12000,\"Status\":\"ACTIVE\",\"WriteUnitsPerSecond\":4000}",
            );

            MockResponse {
                status: response.status,
                headers: response.headers,
                body: Bytes::from(patched),
            }
        })
    }
}

/// Build the service list with the DynamoDB WarmThroughput patch applied.
fn patched_test_services() -> Vec<Arc<dyn MockService>> {
    test_services()
        .into_iter()
        .map(|svc| {
            if svc.service_name() == "dynamodb" {
                Arc::new(DynamoDbWarmThroughputPatch::new(svc)) as Arc<dyn MockService>
            } else {
                svc
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Helper: run a single DynamoDB terraform test with the patched service
// ---------------------------------------------------------------------------

async fn dynamodb_apply(hcl: &str) -> (bool, String, String, String) {
    let port = start_server(patched_test_services()).await;
    let url = format!("http://127.0.0.1:{port}");

    // Use a hash of the HCL for unique dir name
    let hash = {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        hcl.hash(&mut h);
        h.finish()
    };
    let dir = create_tf_dir(&format!("ddb-{hash:016x}"));

    write_provider_tf(&dir, &url);
    std::fs::write(dir.join("main.tf"), hcl).unwrap();

    terraform_init(&dir).await;
    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();

    cleanup_tf_dir(&dir);

    (ok, stdout, stderr, state)
}

// ---------------------------------------------------------------------------
// Success-oriented tests
// ---------------------------------------------------------------------------

/// Basic DynamoDB table with PAY_PER_REQUEST billing and a hash key.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_dynamodb_table_basic() {
    let (ok, _stdout, stderr, state) = dynamodb_apply(
        r#"
resource "aws_dynamodb_table" "basic" {
  name         = "basic-test-table"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "id"

  attribute {
    name = "id"
    type = "S"
  }
}
"#,
    )
    .await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(state.contains("basic-test-table"));
}

/// DynamoDB table with hash key and range key.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_dynamodb_table_hash_and_range_key() {
    let (ok, _stdout, stderr, state) = dynamodb_apply(
        r#"
resource "aws_dynamodb_table" "composite" {
  name         = "composite-key-table"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "pk"
  range_key    = "sk"

  attribute {
    name = "pk"
    type = "S"
  }

  attribute {
    name = "sk"
    type = "S"
  }
}
"#,
    )
    .await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(state.contains("composite-key-table"));
}

/// DynamoDB table with PROVISIONED billing mode.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_dynamodb_table_provisioned() {
    let (ok, _stdout, stderr, state) = dynamodb_apply(
        r#"
resource "aws_dynamodb_table" "provisioned" {
  name         = "provisioned-test-table"
  billing_mode = "PROVISIONED"
  hash_key     = "id"

  read_capacity  = 5
  write_capacity = 5

  attribute {
    name = "id"
    type = "S"
  }
}
"#,
    )
    .await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(state.contains("provisioned-test-table"));
    assert!(state.contains("PROVISIONED"));
}

/// DynamoDB table with tags.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_dynamodb_table_with_tags() {
    let (ok, _stdout, stderr, state) = dynamodb_apply(
        r#"
resource "aws_dynamodb_table" "tagged" {
  name         = "tagged-test-table"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "id"

  attribute {
    name = "id"
    type = "S"
  }

  tags = {
    Environment = "test"
    Project     = "winterbaume"
  }
}
"#,
    )
    .await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(state.contains("tagged-test-table"));
}

/// DynamoDB table with stream enabled.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_dynamodb_table_with_stream() {
    let (ok, _stdout, stderr, state) = dynamodb_apply(
        r#"
resource "aws_dynamodb_table" "streamed" {
  name             = "stream-test-table"
  billing_mode     = "PAY_PER_REQUEST"
  hash_key         = "id"
  stream_enabled   = true
  stream_view_type = "NEW_AND_OLD_IMAGES"

  attribute {
    name = "id"
    type = "S"
  }
}
"#,
    )
    .await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(state.contains("stream-test-table"));
    assert!(state.contains("NEW_AND_OLD_IMAGES"));
}

/// DynamoDB table with a numeric hash key.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_dynamodb_table_numeric_key() {
    let (ok, _stdout, stderr, state) = dynamodb_apply(
        r#"
resource "aws_dynamodb_table" "numeric" {
  name         = "numeric-key-table"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "user_id"

  attribute {
    name = "user_id"
    type = "N"
  }
}
"#,
    )
    .await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(state.contains("numeric-key-table"));
}

/// DynamoDB table item — insert an item into a table.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_dynamodb_table_item() {
    let (ok, _stdout, stderr, state) = dynamodb_apply(
        r#"
resource "aws_dynamodb_table" "items" {
  name         = "items-test-table"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "id"

  attribute {
    name = "id"
    type = "S"
  }
}

resource "aws_dynamodb_table_item" "item1" {
  table_name = aws_dynamodb_table.items.name
  hash_key   = aws_dynamodb_table.items.hash_key

  item = <<ITEM
{
  "id": {"S": "item-001"},
  "name": {"S": "Test Item"},
  "count": {"N": "42"}
}
ITEM
}
"#,
    )
    .await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(state.contains("items-test-table"));
    assert!(state.contains("item-001"));
}

/// DynamoDB table item with composite key (hash + range).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_dynamodb_table_item_composite_key() {
    let (ok, _stdout, stderr, state) = dynamodb_apply(
        r#"
resource "aws_dynamodb_table" "composite_items" {
  name         = "composite-items-table"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "pk"
  range_key    = "sk"

  attribute {
    name = "pk"
    type = "S"
  }

  attribute {
    name = "sk"
    type = "S"
  }
}

resource "aws_dynamodb_table_item" "composite_item" {
  table_name = aws_dynamodb_table.composite_items.name
  hash_key   = aws_dynamodb_table.composite_items.hash_key
  range_key  = aws_dynamodb_table.composite_items.range_key

  item = <<ITEM
{
  "pk": {"S": "user-123"},
  "sk": {"S": "order-456"},
  "total": {"N": "99"}
}
ITEM
}
"#,
    )
    .await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(state.contains("composite-items-table"));
    assert!(state.contains("user-123"));
}
