use crate::harness::*;

// ---------------------------------------------------------------------------
// aws_opensearchserverless_security_policy
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_opensearchserverless_security_policy_encryption() {
    let result = batch_apply(
        r#"
resource "aws_opensearchserverless_security_policy" "aoss_policy_enc" {
  name        = "aoss-enc-basic"
  type        = "encryption"
  description = "encryption policy basic"
  policy = jsonencode({
    Rules = [
      {
        ResourceType = "collection"
        Resource     = ["collection/aoss-enc-basic-*"]
      }
    ]
    AWSOwnedKey = true
  })
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("aoss-enc-basic"));
    assert!(result.state.contains("encryption"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_opensearchserverless_security_policy_network() {
    let result = batch_apply(
        r#"
resource "aws_opensearchserverless_security_policy" "aoss_policy_net" {
  name        = "aoss-net-basic"
  type        = "network"
  description = "network policy basic"
  policy = jsonencode([
    {
      Rules = [
        {
          ResourceType = "collection"
          Resource     = ["collection/aoss-net-basic-*"]
        }
      ]
      AllowFromPublic = true
    }
  ])
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("aoss-net-basic"));
    assert!(result.state.contains("network"));
}

// ---------------------------------------------------------------------------
// aws_opensearchserverless_collection
//
// In real AWS, a collection requires a matching encryption policy to exist
// first; the terraform provider therefore typically reads/expects one at
// create time. Bundle the policy + collection together in a single HCL block
// (with an explicit `depends_on`) so apply succeeds regardless of provider
// ordering.
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_opensearchserverless_collection_basic() {
    let result = batch_apply(
        r#"
resource "aws_opensearchserverless_security_policy" "aoss_collection_basic_enc" {
  name = "aoss-collection-basic-enc"
  type = "encryption"
  policy = jsonencode({
    Rules = [
      {
        ResourceType = "collection"
        Resource     = ["collection/aoss-collection-basic"]
      }
    ]
    AWSOwnedKey = true
  })
}

resource "aws_opensearchserverless_collection" "aoss_collection_basic" {
  name        = "aoss-collection-basic"
  type        = "SEARCH"
  description = "collection basic"

  depends_on = [aws_opensearchserverless_security_policy.aoss_collection_basic_enc]
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("aoss-collection-basic"));
    assert!(result.state.contains("\"type\": \"SEARCH\"") || result.state.contains("SEARCH"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_opensearchserverless_collection_vectorsearch() {
    let result = batch_apply(
        r#"
resource "aws_opensearchserverless_security_policy" "aoss_collection_vector_enc" {
  name = "aoss-collection-vector-enc"
  type = "encryption"
  policy = jsonencode({
    Rules = [
      {
        ResourceType = "collection"
        Resource     = ["collection/aoss-collection-vector"]
      }
    ]
    AWSOwnedKey = true
  })
}

resource "aws_opensearchserverless_collection" "aoss_collection_vector" {
  name        = "aoss-collection-vector"
  type        = "VECTORSEARCH"
  description = "vector search collection"

  tags = {
    Environment = "test"
    Workload    = "vectors"
  }

  depends_on = [aws_opensearchserverless_security_policy.aoss_collection_vector_enc]
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("aoss-collection-vector"));
    assert!(result.state.contains("VECTORSEARCH"));
}

// ---------------------------------------------------------------------------
// Full stack: multiple collections + encryption and network policies together.
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_opensearchserverless_full_stack() {
    let result = batch_apply(
        r#"
resource "aws_opensearchserverless_security_policy" "aoss_full_enc" {
  name = "aoss-full-enc"
  type = "encryption"
  policy = jsonencode({
    Rules = [
      {
        ResourceType = "collection"
        Resource     = ["collection/aoss-full-*"]
      }
    ]
    AWSOwnedKey = true
  })
}

resource "aws_opensearchserverless_security_policy" "aoss_full_net" {
  name = "aoss-full-net"
  type = "network"
  policy = jsonencode([
    {
      Rules = [
        {
          ResourceType = "collection"
          Resource     = ["collection/aoss-full-*"]
        }
      ]
      AllowFromPublic = true
    }
  ])
}

resource "aws_opensearchserverless_collection" "aoss_full_search" {
  name = "aoss-full-search"
  type = "SEARCH"

  depends_on = [
    aws_opensearchserverless_security_policy.aoss_full_enc,
    aws_opensearchserverless_security_policy.aoss_full_net,
  ]
}

resource "aws_opensearchserverless_collection" "aoss_full_timeseries" {
  name = "aoss-full-timeseries"
  type = "TIMESERIES"

  depends_on = [
    aws_opensearchserverless_security_policy.aoss_full_enc,
    aws_opensearchserverless_security_policy.aoss_full_net,
  ]
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("aoss-full-enc"));
    assert!(result.state.contains("aoss-full-net"));
    assert!(result.state.contains("aoss-full-search"));
    assert!(result.state.contains("aoss-full-timeseries"));
    assert!(result.state.contains("TIMESERIES"));
}
