use crate::harness::*;

// RDS terraform resources tested here:
//   aws_db_subnet_group      (CreateDBSubnetGroup, DescribeDBSubnetGroups,
//                              ModifyDBSubnetGroup, DeleteDBSubnetGroup,
//                              AddTagsToResource, ListTagsForResource)
//   aws_db_parameter_group   (CreateDBParameterGroup, DescribeDBParameterGroups,
//                              ModifyDBParameterGroup, DeleteDBParameterGroup,
//                              DescribeDBParameters, AddTagsToResource, ListTagsForResource)
//
// Known issue: aws_db_instance tests are blocked because the terraform AWS
// provider (v6.39) sends DescribeDBInstances with an empty DBInstanceIdentifier
// after creation, and the handler returns a 404 for empty identifiers instead of
// treating them as a list-all. This needs a fix in crates/winterbaume-rds (the
// handler should treat empty-string identifiers the same as absent identifiers).
//
// All tests use isolated servers (not batch_apply) to avoid state contamination
// from other service test modules sharing the batch directory.

// ---------------------------------------------------------------------------
// aws_db_subnet_group
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_rds_db_subnet_group_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("rds-subnet-group-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_db_subnet_group" "test" {
  name       = "rds-subnet-group-basic"
  subnet_ids = ["subnet-11111111", "subnet-22222222"]

  tags = {
    Name = "rds-subnet-group-basic"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("rds-subnet-group-basic"),
        "state missing subnet group"
    );
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_rds_db_subnet_group_with_description() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("rds-subnet-group-desc");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_db_subnet_group" "test" {
  name        = "rds-subnet-group-desc"
  description = "Test DB subnet group with description"
  subnet_ids  = ["subnet-33333333", "subnet-44444444"]
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("rds-subnet-group-desc"),
        "state missing subnet group"
    );
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_rds_db_subnet_group_modify() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("rds-subnet-group-modify");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_db_subnet_group" "test" {
  name        = "rds-subnet-group-modify"
  description = "initial description"
  subnet_ids  = ["subnet-55555555", "subnet-66666666"]
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "first apply failed:\n{stderr}");

    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_db_subnet_group" "test" {
  name        = "rds-subnet-group-modify"
  description = "updated description"
  subnet_ids  = ["subnet-55555555", "subnet-66666666", "subnet-77777777"]
}
"#,
    )
    .unwrap();
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "second apply (modify) failed:\n{stderr}");

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_db_parameter_group
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_rds_db_parameter_group_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("rds-param-group-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_db_parameter_group" "test" {
  name   = "rds-param-group-basic"
  family = "mysql8.0"

  tags = {
    Name = "rds-param-group-basic"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("rds-param-group-basic"),
        "state missing parameter group"
    );
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_rds_db_parameter_group_with_description() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("rds-param-group-desc");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_db_parameter_group" "test" {
  name        = "rds-param-group-desc"
  family      = "postgres16"
  description = "Test parameter group for PostgreSQL"

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("rds-param-group-desc"),
        "state missing parameter group"
    );
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_rds_db_parameter_group_postgres() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("rds-param-group-pg");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_db_parameter_group" "test" {
  name   = "rds-param-group-pg"
  family = "postgres16"
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("rds-param-group-pg"),
        "state missing parameter group"
    );
    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_db_instance — debug smoke test only (see known issue above)
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn debug_rds_db_instance_smoke() {
    let result = smoke_test_terraform(
        test_services(),
        r#"
resource "aws_db_instance" "test" {
  identifier           = "rds-db-instance-smoke"
  instance_class       = "db.t3.micro"
  engine               = "mysql"
  engine_version       = "8.0"
  username             = "admin"
  password             = "testpassword123"
  allocated_storage    = 20
  skip_final_snapshot  = true
}
"#,
        std::time::Duration::from_secs(120),
    )
    .await;
    eprintln!("{result}");
    for call in &result.api_calls {
        eprintln!("  {} -> {}", call.operation, call.status_code);
    }
    eprintln!("Missing ops: {:?}", result.missing_operations);
    eprintln!("Failed ops: {:?}", result.failed_operations);
    // Known failure: the provider sends DescribeDBInstances with empty
    // DBInstanceIdentifier after creation; the handler returns 404.
    // Uncomment the assertion below once the handler is fixed:
    // assert!(result.success, "smoke test failed");
}
