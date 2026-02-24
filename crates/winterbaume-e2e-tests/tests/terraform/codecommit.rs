use crate::harness::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_codecommit_repository_basic() {
    let result = batch_apply(
        r#"
resource "aws_codecommit_repository" "codecommit_repository_basic" {
  repository_name = "terraform-test-repo"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-repo"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_codecommit_repository_with_description() {
    let result = batch_apply(
        r#"
resource "aws_codecommit_repository" "codecommit_repository_with_description" {
  repository_name = "terraform-described-repo"
  description     = "A test repository with a description"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-described-repo"));
    assert!(
        result
            .state
            .contains("A test repository with a description")
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_codecommit_repository_with_tags() {
    let result = batch_apply(
        r#"
resource "aws_codecommit_repository" "codecommit_repository_with_tags" {
  repository_name = "terraform-tagged-repo"
  description     = "A tagged repository"

  tags = {
    Environment = "test"
    Project     = "winterbaume"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-tagged-repo"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_codecommit_repository_with_default_branch() {
    let result = batch_apply(
        r#"
resource "aws_codecommit_repository" "codecommit_repository_with_default_branch" {
  repository_name = "terraform-branch-repo"
  default_branch  = "main"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-branch-repo"));
}
