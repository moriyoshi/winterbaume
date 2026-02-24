use crate::harness::*;

/// Minimal Pass-state state machine definition that Step Functions accepts.
const PASS_STATE_DEFINITION: &str = r#"jsonencode({
    Comment = "Minimal E2E test machine"
    StartAt = "PassState"
    States  = {
      PassState = {
        Type = "Pass"
        End  = true
      }
    }
  })"#;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_sfn_state_machine_standard() {
    let result = batch_apply(&format!(
        r#"
resource "aws_iam_role" "sfn_standard_role" {{
  name = "sfn-standard-test-role"
  assume_role_policy = jsonencode({{
    Version = "2012-10-17"
    Statement = [{{
      Action    = "sts:AssumeRole"
      Effect    = "Allow"
      Principal = {{ Service = "states.amazonaws.com" }}
    }}]
  }})
}}

resource "aws_sfn_state_machine" "sfn_standard" {{
  name     = "terraform-standard-machine"
  role_arn = aws_iam_role.sfn_standard_role.arn
  type     = "STANDARD"

  definition = {PASS_STATE_DEFINITION}

  tags = {{
    Environment = "test"
  }}
}}
"#
    ))
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-standard-machine"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_sfn_state_machine_express() {
    let result = batch_apply(&format!(
        r#"
resource "aws_iam_role" "sfn_express_role" {{
  name = "sfn-express-test-role"
  assume_role_policy = jsonencode({{
    Version = "2012-10-17"
    Statement = [{{
      Action    = "sts:AssumeRole"
      Effect    = "Allow"
      Principal = {{ Service = "states.amazonaws.com" }}
    }}]
  }})
}}

resource "aws_sfn_state_machine" "sfn_express" {{
  name     = "terraform-express-machine"
  role_arn = aws_iam_role.sfn_express_role.arn
  type     = "EXPRESS"

  definition = {PASS_STATE_DEFINITION}
}}
"#
    ))
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-express-machine"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_sfn_activity() {
    let result = batch_apply(
        r#"
resource "aws_sfn_activity" "sfn_activity" {
  name = "terraform-test-activity"

  tags = {
    Purpose = "testing"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-activity"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_sfn_state_machine_modify_in_place() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("sfn-modify");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        format!(
            r#"
resource "aws_iam_role" "sfn_modify_role" {{
  name = "sfn-modify-test-role"
  assume_role_policy = jsonencode({{
    Version = "2012-10-17"
    Statement = [{{
      Action    = "sts:AssumeRole"
      Effect    = "Allow"
      Principal = {{ Service = "states.amazonaws.com" }}
    }}]
  }})
}}

resource "aws_sfn_state_machine" "sfn_modify" {{
  name     = "terraform-modify-machine"
  role_arn = aws_iam_role.sfn_modify_role.arn

  definition = {PASS_STATE_DEFINITION}
}}
"#
        ),
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "first apply failed:\n{stderr}");

    // Update the definition to a slightly different pass state
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_iam_role" "sfn_modify_role" {
  name = "sfn-modify-test-role"
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action    = "sts:AssumeRole"
      Effect    = "Allow"
      Principal = { Service = "states.amazonaws.com" }
    }]
  })
}

resource "aws_sfn_state_machine" "sfn_modify" {
  name     = "terraform-modify-machine"
  role_arn = aws_iam_role.sfn_modify_role.arn

  definition = jsonencode({
    Comment = "Updated definition"
    StartAt = "Pass"
    States  = {
      Pass = {
        Type   = "Pass"
        Result = { updated = true }
        End    = true
      }
    }
  })
}
"#,
    )
    .unwrap();

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "second apply (modify) failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 0 added, 1 changed, 0 destroyed"),
        "Expected in-place update:\n{stdout}"
    );

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_sfn_multiple_activities() {
    let result = batch_apply(
        r#"
resource "aws_sfn_activity" "sfn_activity_alpha" {
  name = "terraform-activity-alpha"

  tags = {
    Purpose = "alpha"
    Env     = "test"
  }
}

resource "aws_sfn_activity" "sfn_activity_beta" {
  name = "terraform-activity-beta"

  tags = {
    Purpose = "beta"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-activity-alpha"));
    assert!(result.state.contains("terraform-activity-beta"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_sfn_state_machine_tag_update() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("sfn-tag-update");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        format!(
            r#"
resource "aws_iam_role" "sfn_tag_role" {{
  name = "sfn-tag-update-role"
  assume_role_policy = jsonencode({{
    Version = "2012-10-17"
    Statement = [{{
      Action    = "sts:AssumeRole"
      Effect    = "Allow"
      Principal = {{ Service = "states.amazonaws.com" }}
    }}]
  }})
}}

resource "aws_sfn_state_machine" "sfn_tag_machine" {{
  name     = "terraform-tag-update-machine"
  role_arn = aws_iam_role.sfn_tag_role.arn

  definition = {PASS_STATE_DEFINITION}

  tags = {{
    Environment = "staging"
    Team        = "platform"
  }}
}}
"#
        ),
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "first apply failed:\n{stderr}");

    // Update tags: change Environment, remove Team, add Version
    std::fs::write(
        dir.join("main.tf"),
        format!(
            r#"
resource "aws_iam_role" "sfn_tag_role" {{
  name = "sfn-tag-update-role"
  assume_role_policy = jsonencode({{
    Version = "2012-10-17"
    Statement = [{{
      Action    = "sts:AssumeRole"
      Effect    = "Allow"
      Principal = {{ Service = "states.amazonaws.com" }}
    }}]
  }})
}}

resource "aws_sfn_state_machine" "sfn_tag_machine" {{
  name     = "terraform-tag-update-machine"
  role_arn = aws_iam_role.sfn_tag_role.arn

  definition = {PASS_STATE_DEFINITION}

  tags = {{
    Environment = "production"
    Version     = "v2"
  }}
}}
"#
        ),
    )
    .unwrap();

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "second apply (tag update) failed:\n{stderr}");
    assert!(
        stdout.contains("1 changed"),
        "Expected in-place tag update:\n{stdout}"
    );

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_sfn_alias_lifecycle() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("sfn-alias");

    write_provider_tf(&dir, &url);

    // Step 1: Create state machine and publish a version
    std::fs::write(
        dir.join("main.tf"),
        format!(
            r#"
resource "aws_iam_role" "sfn_alias_role" {{
  name = "sfn-alias-test-role"
  assume_role_policy = jsonencode({{
    Version = "2012-10-17"
    Statement = [{{
      Action    = "sts:AssumeRole"
      Effect    = "Allow"
      Principal = {{ Service = "states.amazonaws.com" }}
    }}]
  }})
}}

resource "aws_sfn_state_machine" "sfn_alias_machine" {{
  name     = "terraform-alias-machine"
  role_arn = aws_iam_role.sfn_alias_role.arn
  type     = "STANDARD"
  publish  = true

  definition = {PASS_STATE_DEFINITION}
}}

resource "aws_sfn_alias" "sfn_test_alias" {{
  name             = "live"
  description      = "Live traffic alias"

  routing_configuration {{
    state_machine_version_arn = "${{aws_sfn_state_machine.sfn_alias_machine.state_machine_version_arn}}"
    weight                    = 100
  }}
}}
"#
        ),
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "apply (create alias) failed:\n{stderr}");

    // Verify all resources exist in state
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("terraform-alias-machine"),
        "state machine not found in state"
    );
    assert!(
        state.contains("aws_sfn_alias"),
        "alias resource not found in state"
    );

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_sfn_state_machine_destroy_and_recreate() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("sfn-destroy-recreate");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        format!(
            r#"
resource "aws_iam_role" "sfn_recreate_role" {{
  name = "sfn-recreate-test-role"
  assume_role_policy = jsonencode({{
    Version = "2012-10-17"
    Statement = [{{
      Action    = "sts:AssumeRole"
      Effect    = "Allow"
      Principal = {{ Service = "states.amazonaws.com" }}
    }}]
  }})
}}

resource "aws_sfn_state_machine" "sfn_recreate" {{
  name     = "terraform-recreate-machine"
  role_arn = aws_iam_role.sfn_recreate_role.arn

  definition = {PASS_STATE_DEFINITION}
}}
"#
        ),
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "first apply failed:\n{stderr}");

    // Destroy: remove the state machine but keep the role
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_iam_role" "sfn_recreate_role" {
  name = "sfn-recreate-test-role"
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action    = "sts:AssumeRole"
      Effect    = "Allow"
      Principal = { Service = "states.amazonaws.com" }
    }]
  })
}
"#,
    )
    .unwrap();

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "destroy apply failed:\n{stderr}");
    assert!(
        stdout.contains("1 destroyed"),
        "Expected 1 resource destroyed:\n{stdout}"
    );

    // Recreate: add the state machine back with a different definition
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_iam_role" "sfn_recreate_role" {
  name = "sfn-recreate-test-role"
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action    = "sts:AssumeRole"
      Effect    = "Allow"
      Principal = { Service = "states.amazonaws.com" }
    }]
  })
}

resource "aws_sfn_state_machine" "sfn_recreate" {
  name     = "terraform-recreate-machine"
  role_arn = aws_iam_role.sfn_recreate_role.arn

  definition = jsonencode({
    Comment = "Recreated machine"
    StartAt = "Done"
    States  = {
      Done = {
        Type = "Pass"
        End  = true
      }
    }
  })
}
"#,
    )
    .unwrap();

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "recreate apply failed:\n{stderr}");
    assert!(
        stdout.contains("1 added"),
        "Expected 1 resource added on recreate:\n{stdout}"
    );

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_sfn_state_machine_with_logging() {
    let result = batch_apply(&format!(
        r#"
resource "aws_cloudwatch_log_group" "sfn_logs" {{
  name = "/aws/sfn/test-state-machine"
}}

resource "aws_iam_role" "sfn_logging_role" {{
  name = "sfn-logging-test-role"
  assume_role_policy = jsonencode({{
    Version = "2012-10-17"
    Statement = [{{
      Action    = "sts:AssumeRole"
      Effect    = "Allow"
      Principal = {{ Service = "states.amazonaws.com" }}
    }}]
  }})
}}

resource "aws_sfn_state_machine" "test_logging" {{
  name     = "test-state-machine-logging"
  role_arn = aws_iam_role.sfn_logging_role.arn

  definition = {PASS_STATE_DEFINITION}

  logging_configuration {{
    level                  = "ALL"
    include_execution_data = true

    log_destination = "${{aws_cloudwatch_log_group.sfn_logs.arn}}:*"
  }}

  tracing_configuration {{
    enabled = true
  }}
}}
"#
    ))
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("test-state-machine-logging"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_sfn_full_stack() {
    let result = batch_apply(&format!(
        r#"
resource "aws_iam_role" "sfn_full_role" {{
  name = "sfn-full-stack-role"
  assume_role_policy = jsonencode({{
    Version = "2012-10-17"
    Statement = [{{
      Action    = "sts:AssumeRole"
      Effect    = "Allow"
      Principal = {{ Service = "states.amazonaws.com" }}
    }}]
  }})

  tags = {{
    Stack = "full"
  }}
}}

resource "aws_sfn_state_machine" "sfn_full_machine" {{
  name     = "sfn-full-stack-machine"
  role_arn = aws_iam_role.sfn_full_role.arn

  definition = {PASS_STATE_DEFINITION}

  tags = {{
    Stack = "full"
  }}
}}

resource "aws_sfn_activity" "sfn_full_activity" {{
  name = "sfn-full-stack-activity"

  tags = {{
    Stack = "full"
  }}
}}
"#
    ))
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("sfn-full-stack-machine"));
    assert!(result.state.contains("sfn-full-stack-activity"));
    assert!(result.state.contains("sfn-full-stack-role"));
}
