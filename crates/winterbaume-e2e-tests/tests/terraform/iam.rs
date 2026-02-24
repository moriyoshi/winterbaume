use crate::harness::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_iam_user() {
    let result = batch_apply(
        r#"
resource "aws_iam_user" "iam_user" {
  name = "terraform-test-user"
  path = "/engineers/"

  tags = {
    Team = "platform"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-user"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_iam_role() {
    let result = batch_apply(
        r#"
resource "aws_iam_role" "iam_role" {
  name = "terraform-test-role"
  path = "/service-role/"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action    = "sts:AssumeRole"
      Effect    = "Allow"
      Principal = { Service = "lambda.amazonaws.com" }
    }]
  })

  max_session_duration = 7200

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-role"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_iam_policy() {
    let result = batch_apply(
        r#"
resource "aws_iam_policy" "iam_policy" {
  name        = "terraform-test-policy"
  path        = "/app/"
  description = "Test policy for terraform E2E"

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action   = ["s3:GetObject", "s3:ListBucket"]
      Effect   = "Allow"
      Resource = "*"
    }]
  })

  tags = {
    Purpose = "testing"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-policy"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_iam_group() {
    let result = batch_apply(
        r#"
resource "aws_iam_group" "iam_group" {
  name = "terraform-test-group"
  path = "/teams/"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-group"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_iam_instance_profile() {
    let result = batch_apply(
        r#"
resource "aws_iam_role" "iam_instance_profile" {
  name = "instance-profile-test-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action    = "sts:AssumeRole"
      Effect    = "Allow"
      Principal = { Service = "ec2.amazonaws.com" }
    }]
  })
}

resource "aws_iam_instance_profile" "iam_instance_profile" {
  name = "terraform-test-instance-profile"
  role = aws_iam_role.iam_instance_profile.name

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-instance-profile"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_iam_role_policy_attachment() {
    let result = batch_apply(
        r#"
resource "aws_iam_role" "iam_role_policy_attach" {
  name = "role-attach-test-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action    = "sts:AssumeRole"
      Effect    = "Allow"
      Principal = { Service = "lambda.amazonaws.com" }
    }]
  })
}

resource "aws_iam_policy" "iam_role_policy_attach" {
  name = "role-attach-test-policy"

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action   = "logs:*"
      Effect   = "Allow"
      Resource = "*"
    }]
  })
}

resource "aws_iam_role_policy_attachment" "iam_role_policy_attach" {
  role       = aws_iam_role.iam_role_policy_attach.name
  policy_arn = aws_iam_policy.iam_role_policy_attach.arn
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("role-attach-test-role"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_iam_user_policy_attachment() {
    let result = batch_apply(
        r#"
resource "aws_iam_user" "iam_user_policy_attach" {
  name = "user-attach-test-user"
}

resource "aws_iam_policy" "iam_user_policy_attach" {
  name = "user-attach-test-policy"

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action   = "s3:GetObject"
      Effect   = "Allow"
      Resource = "*"
    }]
  })
}

resource "aws_iam_user_policy_attachment" "iam_user_policy_attach" {
  user       = aws_iam_user.iam_user_policy_attach.name
  policy_arn = aws_iam_policy.iam_user_policy_attach.arn
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("user-attach-test-user"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_iam_role_inline_policy() {
    let result = batch_apply(
        r#"
resource "aws_iam_role" "iam_role_inline" {
  name = "inline-policy-test-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action    = "sts:AssumeRole"
      Effect    = "Allow"
      Principal = { Service = "lambda.amazonaws.com" }
    }]
  })
}

resource "aws_iam_role_policy" "iam_role_inline" {
  name = "inline-test-policy"
  role = aws_iam_role.iam_role_inline.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action   = "logs:CreateLogGroup"
      Effect   = "Allow"
      Resource = "*"
    }]
  })
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("inline-policy-test-role"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_iam_access_key() {
    let result = batch_apply(
        r#"
resource "aws_iam_user" "iam_access_key" {
  name = "access-key-test-user"
}

resource "aws_iam_access_key" "iam_access_key" {
  user = aws_iam_user.iam_access_key.name
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("access-key-test-user"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_iam_group_membership() {
    let result = batch_apply(
        r#"
resource "aws_iam_group" "iam_group_membership" {
  name = "membership-test-group"
}

resource "aws_iam_user" "iam_membership_alice" {
  name = "alice"
}

resource "aws_iam_user" "iam_membership_bob" {
  name = "bob"
}

resource "aws_iam_group_membership" "iam_group_membership" {
  name  = "test-membership"
  group = aws_iam_group.iam_group_membership.name
  users = [
    aws_iam_user.iam_membership_alice.name,
    aws_iam_user.iam_membership_bob.name,
  ]
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("membership-test-group"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_iam_group_policy_attachment() {
    let result = batch_apply(
        r#"
resource "aws_iam_group" "iam_group_policy_attach" {
  name = "group-attach-test"
}

resource "aws_iam_policy" "iam_group_policy_attach" {
  name = "group-attach-test-policy"

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action   = "s3:ListAllMyBuckets"
      Effect   = "Allow"
      Resource = "*"
    }]
  })
}

resource "aws_iam_group_policy_attachment" "iam_group_policy_attach" {
  group      = aws_iam_group.iam_group_policy_attach.name
  policy_arn = aws_iam_policy.iam_group_policy_attach.arn
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("group-attach-test"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_iam_full_stack() {
    let result = batch_apply(
        r#"
resource "aws_iam_user" "iam_full_dev" {
  name = "developer"
  path = "/engineering/"

  tags = {
    Team = "backend"
  }
}

resource "aws_iam_group" "iam_full_devs" {
  name = "developers"
}

resource "aws_iam_group_membership" "iam_full_devs" {
  name  = "devs-membership"
  group = aws_iam_group.iam_full_devs.name
  users = [aws_iam_user.iam_full_dev.name]
}

resource "aws_iam_role" "iam_full_app" {
  name = "app-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action    = "sts:AssumeRole"
      Effect    = "Allow"
      Principal = { Service = "ecs-tasks.amazonaws.com" }
    }]
  })

  tags = {
    Service = "app"
  }
}

resource "aws_iam_policy" "iam_full_s3" {
  name        = "s3-read-policy"
  description = "Allow reading from S3"

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action   = ["s3:GetObject", "s3:ListBucket"]
      Effect   = "Allow"
      Resource = "*"
    }]
  })
}

resource "aws_iam_role_policy_attachment" "iam_full_app_s3" {
  role       = aws_iam_role.iam_full_app.name
  policy_arn = aws_iam_policy.iam_full_s3.arn
}

resource "aws_iam_instance_profile" "iam_full_app" {
  name = "app-instance-profile"
  role = aws_iam_role.iam_full_app.name
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("developer"));
    assert!(result.state.contains("developers"));
    assert!(result.state.contains("app-role"));
    assert!(result.state.contains("s3-read-policy"));
    assert!(result.state.contains("app-instance-profile"));
}
