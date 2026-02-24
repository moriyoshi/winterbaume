use crate::harness::*;

// Organizations terraform resources tested here:
//   aws_organizations_organization        (CreateOrganization, DescribeOrganization,
//                                          ListRoots, DeleteOrganization)
//   aws_organizations_organizational_unit (CreateOrganizationalUnit, DescribeOrganizationalUnit,
//                                          UpdateOrganizationalUnit, DeleteOrganizationalUnit)
//   aws_organizations_policy              (CreatePolicy, DescribePolicy, UpdatePolicy,
//                                          DeletePolicy, EnablePolicyType)
//   aws_organizations_policy_attachment   (AttachPolicy, DetachPolicy)
//
// Note: Each test uses an isolated server because Organizations state is global
// (only one organization per account) and tests would conflict in batch_apply.

// ---------------------------------------------------------------------------
// aws_organizations_organization
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_organizations_organization_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("org-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_organizations_organization" "org_basic" {
  feature_set = "ALL"
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_organizations_organization"),
        "state missing organization"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_organizations_organizational_unit
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_organizations_ou_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("org-ou-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_organizations_organization" "org_ou" {
  feature_set = "ALL"
}

resource "aws_organizations_organizational_unit" "org_ou_basic" {
  name      = "org-ou-basic"
  parent_id = aws_organizations_organization.org_ou.roots[0].id
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("org-ou-basic"), "state missing OU");

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_organizations_policy
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_organizations_policy_scp() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("org-policy-scp");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_organizations_organization" "org_policy" {
  feature_set = "ALL"
}

resource "aws_organizations_policy" "org_policy_scp" {
  name        = "org-policy-scp"
  description = "Deny all EC2 actions"
  type        = "SERVICE_CONTROL_POLICY"

  content = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect   = "Deny"
        Action   = ["ec2:*"]
        Resource = "*"
      }
    ]
  })

  depends_on = [aws_organizations_organization.org_policy]
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("org-policy-scp"), "state missing policy");

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_organizations_policy_attachment
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_organizations_policy_attachment() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("org-policy-attach");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_organizations_organization" "org_attach" {
  feature_set = "ALL"
}

resource "aws_organizations_organizational_unit" "org_attach_ou" {
  name      = "org-attach-ou"
  parent_id = aws_organizations_organization.org_attach.roots[0].id
}

resource "aws_organizations_policy" "org_attach_policy" {
  name    = "org-attach-policy"
  type    = "SERVICE_CONTROL_POLICY"
  content = jsonencode({
    Version = "2012-10-17"
    Statement = [{ Effect = "Allow", Action = "*", Resource = "*" }]
  })

  depends_on = [aws_organizations_organization.org_attach]
}

resource "aws_organizations_policy_attachment" "org_attach_attach" {
  policy_id = aws_organizations_policy.org_attach_policy.id
  target_id = aws_organizations_organizational_unit.org_attach_ou.id
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_organizations_policy_attachment"),
        "state missing policy attachment"
    );

    cleanup_tf_dir(&dir);
}
