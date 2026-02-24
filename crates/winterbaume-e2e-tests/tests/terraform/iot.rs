use crate::harness::*;

// ---------------------------------------------------------------------------
// aws_iot_thing
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_iot_thing_basic() {
    let result = batch_apply(
        r#"
resource "aws_iot_thing" "iot_thing_basic" {
  name = "iot-thing-basic"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("iot-thing-basic"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_iot_thing_with_attributes() {
    let result = batch_apply(
        r#"
resource "aws_iot_thing" "iot_thing_with_attributes" {
  name = "iot-thing-with-attributes"

  attributes = {
    First  = "alpha"
    Second = "beta"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("iot-thing-with-attributes"));
    assert!(result.state.contains("alpha"));
}

// ---------------------------------------------------------------------------
// aws_iot_thing_type
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_iot_thing_type_basic() {
    let result = batch_apply(
        r#"
resource "aws_iot_thing_type" "iot_thing_type_basic" {
  name = "iot-thing-type-basic"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("iot-thing-type-basic"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_iot_thing_type_with_properties() {
    let result = batch_apply(
        r#"
resource "aws_iot_thing_type" "iot_thing_type_with_properties" {
  name = "iot-thing-type-with-properties"

  properties {
    description           = "Sensor type with searchable attributes"
    searchable_attributes = ["model", "serial"]
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("iot-thing-type-with-properties"));
    assert!(
        result
            .state
            .contains("Sensor type with searchable attributes")
    );
}

// ---------------------------------------------------------------------------
// aws_iot_policy
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_iot_policy_basic() {
    let result = batch_apply(
        r#"
resource "aws_iot_policy" "iot_policy_basic" {
  name = "iot-policy-basic"

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect   = "Allow"
        Action   = ["iot:Connect"]
        Resource = ["*"]
      }
    ]
  })
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("iot-policy-basic"));
    assert!(result.state.contains("iot:Connect"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_iot_policy_publish() {
    let result = batch_apply(
        r#"
resource "aws_iot_policy" "iot_policy_publish" {
  name = "iot-policy-publish"

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect   = "Allow"
        Action   = ["iot:Publish", "iot:Subscribe", "iot:Receive"]
        Resource = ["*"]
      }
    ]
  })
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("iot-policy-publish"));
    assert!(result.state.contains("iot:Publish"));
}

// ---------------------------------------------------------------------------
// Full stack: thing type + thing referencing it + policy
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_iot_full_stack() {
    let result = batch_apply(
        r#"
resource "aws_iot_thing_type" "iot_full_stack" {
  name = "iot-full-stack-type"

  properties {
    description           = "Type used by full-stack test"
    searchable_attributes = ["region"]
  }
}

resource "aws_iot_thing" "iot_full_stack" {
  name            = "iot-full-stack-thing"
  thing_type_name = aws_iot_thing_type.iot_full_stack.name

  attributes = {
    region = "eu"
  }
}

resource "aws_iot_policy" "iot_full_stack" {
  name = "iot-full-stack-policy"

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect   = "Allow"
        Action   = ["iot:*"]
        Resource = ["*"]
      }
    ]
  })
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("iot-full-stack-type"));
    assert!(result.state.contains("iot-full-stack-thing"));
    assert!(result.state.contains("iot-full-stack-policy"));
}
