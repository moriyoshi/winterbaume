//! Terraform E2E tests for Amazon Keyspaces (`aws_keyspaces_*` resources).
//!
//! Coverage: `aws_keyspaces_keyspace`, `aws_keyspaces_table`.

use crate::harness::*;

// ---------------------------------------------------------------------------
// aws_keyspaces_keyspace
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_keyspaces_keyspace_basic() {
    let result = batch_apply(
        r#"
resource "aws_keyspaces_keyspace" "keyspaces_keyspace_basic" {
  name = "keyspaces_keyspace_basic"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("keyspaces_keyspace_basic"));
    assert!(result.state.contains("arn:aws:cassandra"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_keyspaces_keyspace_with_tags() {
    let result = batch_apply(
        r#"
resource "aws_keyspaces_keyspace" "keyspaces_keyspace_tagged" {
  name = "keyspaces_keyspace_tagged"

  tags = {
    Environment = "test"
    Project     = "winterbaume-keyspaces"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("keyspaces_keyspace_tagged"));
    assert!(result.state.contains("winterbaume-keyspaces"));
}

// ---------------------------------------------------------------------------
// aws_keyspaces_table
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_keyspaces_table_basic() {
    let result = batch_apply(
        r#"
resource "aws_keyspaces_keyspace" "keyspaces_table_basic_ks" {
  name = "keyspaces_table_basic_ks"
}

resource "aws_keyspaces_table" "keyspaces_table_basic" {
  keyspace_name = aws_keyspaces_keyspace.keyspaces_table_basic_ks.name
  table_name    = "table_basic"

  schema_definition {
    column {
      name = "id"
      type = "uuid"
    }

    partition_key {
      name = "id"
    }
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("keyspaces_table_basic_ks"));
    assert!(result.state.contains("table_basic"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_keyspaces_table_composite_key() {
    let result = batch_apply(
        r#"
resource "aws_keyspaces_keyspace" "keyspaces_table_composite_ks" {
  name = "keyspaces_table_composite_ks"
}

resource "aws_keyspaces_table" "keyspaces_table_composite" {
  keyspace_name = aws_keyspaces_keyspace.keyspaces_table_composite_ks.name
  table_name    = "table_composite"

  schema_definition {
    column {
      name = "user_id"
      type = "uuid"
    }

    column {
      name = "created_at"
      type = "timestamp"
    }

    column {
      name = "payload"
      type = "text"
    }

    partition_key {
      name = "user_id"
    }

    clustering_key {
      name     = "created_at"
      order_by = "DESC"
    }
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("keyspaces_table_composite_ks"));
    assert!(result.state.contains("table_composite"));
    assert!(result.state.contains("created_at"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_keyspaces_table_with_tags() {
    let result = batch_apply(
        r#"
resource "aws_keyspaces_keyspace" "keyspaces_table_tagged_ks" {
  name = "keyspaces_table_tagged_ks"
}

resource "aws_keyspaces_table" "keyspaces_table_tagged" {
  keyspace_name = aws_keyspaces_keyspace.keyspaces_table_tagged_ks.name
  table_name    = "table_tagged"

  schema_definition {
    column {
      name = "id"
      type = "uuid"
    }

    partition_key {
      name = "id"
    }
  }

  tags = {
    Owner   = "winterbaume-keyspaces-team"
    Purpose = "e2e"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("keyspaces_table_tagged_ks"));
    assert!(result.state.contains("table_tagged"));
    assert!(result.state.contains("winterbaume-keyspaces-team"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_keyspaces_table_with_static_column() {
    let result = batch_apply(
        r#"
resource "aws_keyspaces_keyspace" "keyspaces_table_static_ks" {
  name = "keyspaces_table_static_ks"
}

resource "aws_keyspaces_table" "keyspaces_table_static" {
  keyspace_name = aws_keyspaces_keyspace.keyspaces_table_static_ks.name
  table_name    = "table_static"

  schema_definition {
    column {
      name = "user_id"
      type = "uuid"
    }

    column {
      name = "event_id"
      type = "uuid"
    }

    column {
      name = "shared_label"
      type = "text"
    }

    partition_key {
      name = "user_id"
    }

    clustering_key {
      name     = "event_id"
      order_by = "ASC"
    }

    static_column {
      name = "shared_label"
    }
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("keyspaces_table_static_ks"));
    assert!(result.state.contains("table_static"));
    assert!(result.state.contains("shared_label"));
}
