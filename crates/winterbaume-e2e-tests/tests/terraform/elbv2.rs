use crate::harness::*;

// ELBv2 terraform resources tested here:
//   aws_lb                       (CreateLoadBalancer, DescribeLoadBalancers, ModifyLoadBalancerAttributes,
//                                  DescribeLoadBalancerAttributes, SetSecurityGroups, DeleteLoadBalancer,
//                                  AddTags, DescribeTags)
//   aws_lb_target_group          (CreateTargetGroup, DescribeTargetGroups, ModifyTargetGroup,
//                                  ModifyTargetGroupAttributes, DeleteTargetGroup)
//   aws_lb_listener              (CreateListener, DescribeListeners, ModifyListener, DeleteListener)
//   aws_lb_listener_rule         (CreateRule, DescribeRules, ModifyRule, SetRulePriorities, DeleteRule)
//   aws_lb_target_group_attachment (RegisterTargets, DeregisterTargets, DescribeTargetHealth)
//   aws_lb_trust_store           (CreateTrustStore, DescribeTrustStores, ModifyTrustStore, DeleteTrustStore)
//
// Not tested (require VPC/subnet resources not implemented):
//   aws_lb with subnets     — VPC/subnet resources not available in mock

// ---------------------------------------------------------------------------
// aws_lb_target_group
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_elbv2_target_group_basic() {
    let result = batch_apply(
        r#"
resource "aws_lb_target_group" "elbv2_tg_basic" {
  name        = "elbv2-tg-basic"
  port        = 80
  protocol    = "HTTP"
  target_type = "ip"
  vpc_id      = "vpc-12345678"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("elbv2-tg-basic"),
        "state missing target group"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_elbv2_target_group_https() {
    let result = batch_apply(
        r#"
resource "aws_lb_target_group" "elbv2_tg_https" {
  name        = "elbv2-tg-https"
  port        = 443
  protocol    = "HTTPS"
  target_type = "ip"
  vpc_id      = "vpc-12345678"

  health_check {
    path                = "/health"
    interval            = 30
    timeout             = 5
    healthy_threshold   = 3
    unhealthy_threshold = 3
    matcher             = "200"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("elbv2-tg-https"),
        "state missing target group"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_elbv2_target_group_with_tags() {
    let result = batch_apply(
        r#"
resource "aws_lb_target_group" "elbv2_tg_tagged" {
  name        = "elbv2-tg-tagged"
  port        = 8080
  protocol    = "HTTP"
  target_type = "instance"
  vpc_id      = "vpc-12345678"

  tags = {
    Environment = "test"
    Name        = "elbv2-tg-tagged"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("elbv2-tg-tagged"),
        "state missing target group"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_elbv2_target_group_lambda() {
    let result = batch_apply(
        r#"
resource "aws_lb_target_group" "elbv2_tg_lambda" {
  name        = "elbv2-tg-lambda"
  target_type = "lambda"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("elbv2-tg-lambda"),
        "state missing target group"
    );
}

// ---------------------------------------------------------------------------
// aws_lb (load balancer)
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_elbv2_lb_basic() {
    // Isolated server: the provider calls DescribeLoadBalancerAttributes on
    // every refresh and stores computed attributes in state.
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("elbv2-lb-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_lb" "elbv2_lb_basic" {
  name               = "elbv2-lb-basic"
  internal           = false
  load_balancer_type = "application"
  subnets            = ["subnet-12345678", "subnet-87654321"]

  enable_deletion_protection = false

  tags = {
    Name = "elbv2-lb-basic"
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
        state.contains("elbv2-lb-basic"),
        "state missing load balancer"
    );

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_elbv2_lb_network() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("elbv2-lb-network");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_lb" "elbv2_lb_network" {
  name               = "elbv2-lb-network"
  internal           = true
  load_balancer_type = "network"
  subnets            = ["subnet-12345678", "subnet-87654321"]

  tags = {
    Name = "elbv2-lb-network"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("elbv2-lb-network"), "state missing NLB");

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_lb_listener
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_elbv2_listener_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("elbv2-listener-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_lb" "elbv2_listener_lb" {
  name               = "elbv2-listener-lb"
  internal           = false
  load_balancer_type = "application"
  subnets            = ["subnet-12345678", "subnet-87654321"]
}

resource "aws_lb_target_group" "elbv2_listener_tg" {
  name        = "elbv2-listener-tg"
  port        = 80
  protocol    = "HTTP"
  target_type = "ip"
  vpc_id      = "vpc-12345678"
}

resource "aws_lb_listener" "elbv2_listener_basic" {
  load_balancer_arn = aws_lb.elbv2_listener_lb.arn
  port              = 80
  protocol          = "HTTP"

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.elbv2_listener_tg.arn
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
        state.contains("elbv2-listener-lb"),
        "state missing load balancer"
    );
    assert!(
        state.contains("elbv2-listener-tg"),
        "state missing target group"
    );
    assert!(state.contains("aws_lb_listener"), "state missing listener");

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_lb_listener_rule
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_elbv2_listener_rule_path() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("elbv2-rule-path");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_lb" "elbv2_rule_lb" {
  name               = "elbv2-rule-lb"
  internal           = false
  load_balancer_type = "application"
  subnets            = ["subnet-12345678", "subnet-87654321"]
}

resource "aws_lb_target_group" "elbv2_rule_tg_default" {
  name        = "elbv2-rule-tg-default"
  port        = 80
  protocol    = "HTTP"
  target_type = "ip"
  vpc_id      = "vpc-12345678"
}

resource "aws_lb_target_group" "elbv2_rule_tg_api" {
  name        = "elbv2-rule-tg-api"
  port        = 8080
  protocol    = "HTTP"
  target_type = "ip"
  vpc_id      = "vpc-12345678"
}

resource "aws_lb_listener" "elbv2_rule_listener" {
  load_balancer_arn = aws_lb.elbv2_rule_lb.arn
  port              = 80
  protocol          = "HTTP"

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.elbv2_rule_tg_default.arn
  }
}

resource "aws_lb_listener_rule" "elbv2_rule_path" {
  listener_arn = aws_lb_listener.elbv2_rule_listener.arn
  priority     = 100

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.elbv2_rule_tg_api.arn
  }

  condition {
    path_pattern {
      values = ["/api/*"]
    }
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
        state.contains("aws_lb_listener_rule"),
        "state missing listener rule"
    );
    assert!(state.contains("/api/*"), "state missing path pattern");

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_elbv2_listener_rule_host_header() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("elbv2-rule-host");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_lb" "elbv2_rule_host_lb" {
  name               = "elbv2-rule-host-lb"
  internal           = false
  load_balancer_type = "application"
  subnets            = ["subnet-12345678", "subnet-87654321"]
}

resource "aws_lb_target_group" "elbv2_rule_host_tg_default" {
  name        = "elbv2-rule-host-tgdef"
  port        = 80
  protocol    = "HTTP"
  target_type = "ip"
  vpc_id      = "vpc-12345678"
}

resource "aws_lb_target_group" "elbv2_rule_host_tg_app" {
  name        = "elbv2-rule-host-tgapp"
  port        = 8080
  protocol    = "HTTP"
  target_type = "ip"
  vpc_id      = "vpc-12345678"
}

resource "aws_lb_listener" "elbv2_rule_host_listener" {
  load_balancer_arn = aws_lb.elbv2_rule_host_lb.arn
  port              = 80
  protocol          = "HTTP"

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.elbv2_rule_host_tg_default.arn
  }
}

resource "aws_lb_listener_rule" "elbv2_rule_host_header" {
  listener_arn = aws_lb_listener.elbv2_rule_host_listener.arn
  priority     = 200

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.elbv2_rule_host_tg_app.arn
  }

  condition {
    host_header {
      values = ["app.example.com"]
    }
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
        state.contains("aws_lb_listener_rule"),
        "state missing listener rule"
    );
    assert!(
        state.contains("app.example.com"),
        "state missing host header condition"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_lb_target_group_attachment
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_elbv2_target_group_attachment_ip() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("elbv2-tga-ip");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_lb_target_group" "elbv2_tga_ip_tg" {
  name        = "elbv2-tga-ip-tg"
  port        = 80
  protocol    = "HTTP"
  target_type = "ip"
  vpc_id      = "vpc-12345678"
}

resource "aws_lb_target_group_attachment" "elbv2_tga_ip" {
  target_group_arn = aws_lb_target_group.elbv2_tga_ip_tg.arn
  target_id        = "10.0.1.100"
  port             = 8080
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_lb_target_group_attachment"),
        "state missing target group attachment"
    );
    assert!(
        state.contains("10.0.1.100"),
        "state missing target IP address"
    );

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_elbv2_target_group_attachment_instance() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("elbv2-tga-instance");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_lb_target_group" "elbv2_tga_inst_tg" {
  name        = "elbv2-tga-inst-tg"
  port        = 80
  protocol    = "HTTP"
  target_type = "instance"
  vpc_id      = "vpc-12345678"
}

resource "aws_lb_target_group_attachment" "elbv2_tga_instance" {
  target_group_arn = aws_lb_target_group.elbv2_tga_inst_tg.arn
  target_id        = "i-0123456789abcdef0"
  port             = 80
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_lb_target_group_attachment"),
        "state missing target group attachment"
    );
    assert!(
        state.contains("i-0123456789abcdef0"),
        "state missing target instance ID"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_lb_trust_store
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_elbv2_trust_store_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("elbv2-trust-store");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_lb_trust_store" "elbv2_trust_store_basic" {
  name                             = "elbv2-ts-basic"
  ca_certificates_bundle_s3_bucket = "my-trust-store-bucket"
  ca_certificates_bundle_s3_key    = "ca-bundle.pem"

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
        state.contains("aws_lb_trust_store"),
        "state missing trust store"
    );
    assert!(
        state.contains("elbv2-ts-basic"),
        "state missing trust store name"
    );

    cleanup_tf_dir(&dir);
}
