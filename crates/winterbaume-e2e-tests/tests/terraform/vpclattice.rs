use crate::harness::*;

// VPC Lattice terraform resources tested here:
//   aws_vpclattice_service_network                 (CreateServiceNetwork, GetServiceNetwork,
//                                                   UpdateServiceNetwork, DeleteServiceNetwork,
//                                                   ListServiceNetworks, ListTagsForResource, TagResource)
//   aws_vpclattice_service                         (CreateService, GetService, UpdateService,
//                                                   DeleteService, ListServices, ListTagsForResource,
//                                                   TagResource)
//   aws_vpclattice_target_group                    (CreateTargetGroup, GetTargetGroup,
//                                                   UpdateTargetGroup, DeleteTargetGroup,
//                                                   ListTargetGroups, ListTagsForResource, TagResource)
//   aws_vpclattice_listener                        (CreateListener, GetListener, UpdateListener,
//                                                   DeleteListener, ListListeners, ListTagsForResource)
//   aws_vpclattice_listener_rule                   (CreateRule, GetRule, UpdateRule, DeleteRule,
//                                                   ListRules)
//   aws_vpclattice_service_network_service_association (CreateServiceNetworkServiceAssociation,
//                                                       GetServiceNetworkServiceAssociation,
//                                                       DeleteServiceNetworkServiceAssociation)
//   aws_vpclattice_service_network_vpc_association (CreateServiceNetworkVpcAssociation,
//                                                   GetServiceNetworkVpcAssociation,
//                                                   DeleteServiceNetworkVpcAssociation)
//   aws_vpclattice_auth_policy                     (PutAuthPolicy, GetAuthPolicy, DeleteAuthPolicy)
//   aws_vpclattice_resource_policy                 (PutResourcePolicy, GetResourcePolicy,
//                                                   DeleteResourcePolicy)
//
// Not tested:
//   aws_vpclattice_access_log_subscription — requires a real destination ARN.

// ---------------------------------------------------------------------------
// aws_vpclattice_service_network
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_vpclattice_service_network_basic() {
    let result = batch_apply(
        r#"
resource "aws_vpclattice_service_network" "vpclattice_sn_basic" {
  name = "vpclattice-sn-basic"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("vpclattice-sn-basic"),
        "state missing service network"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_vpclattice_service_network_with_auth() {
    let result = batch_apply(
        r#"
resource "aws_vpclattice_service_network" "vpclattice_sn_auth" {
  name      = "vpclattice-sn-auth"
  auth_type = "AWS_IAM"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("vpclattice-sn-auth"),
        "state missing service network"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_vpclattice_service_network_with_tags() {
    let result = batch_apply(
        r#"
resource "aws_vpclattice_service_network" "vpclattice_sn_tags" {
  name = "vpclattice-sn-tags"

  tags = {
    Environment = "test"
    Service     = "vpclattice"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("vpclattice-sn-tags"),
        "state missing service network"
    );
}

// ---------------------------------------------------------------------------
// aws_vpclattice_service
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_vpclattice_service_basic() {
    let result = batch_apply(
        r#"
resource "aws_vpclattice_service" "vpclattice_svc_basic" {
  name = "vpclattice-svc-basic"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("vpclattice-svc-basic"),
        "state missing service"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_vpclattice_service_with_auth() {
    let result = batch_apply(
        r#"
resource "aws_vpclattice_service" "vpclattice_svc_auth" {
  name      = "vpclattice-svc-auth"
  auth_type = "AWS_IAM"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("vpclattice-svc-auth"),
        "state missing service"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_vpclattice_service_with_tags() {
    let result = batch_apply(
        r#"
resource "aws_vpclattice_service" "vpclattice_svc_tags" {
  name = "vpclattice-svc-tags"

  tags = {
    Environment = "test"
    Purpose     = "e2e"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("vpclattice-svc-tags"),
        "state missing service"
    );
}

// ---------------------------------------------------------------------------
// aws_vpclattice_target_group
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_vpclattice_target_group_ip() {
    let result = batch_apply(
        r#"
resource "aws_vpclattice_target_group" "vpclattice_tg_ip" {
  name = "vpclattice-tg-ip"
  type = "IP"

  config {
    port           = 80
    protocol       = "HTTP"
    vpc_identifier = "vpc-12345678"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("vpclattice-tg-ip"),
        "state missing target group"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_vpclattice_target_group_instance() {
    let result = batch_apply(
        r#"
resource "aws_vpclattice_target_group" "vpclattice_tg_instance" {
  name = "vpclattice-tg-instance"
  type = "INSTANCE"

  config {
    port           = 443
    protocol       = "HTTPS"
    vpc_identifier = "vpc-12345678"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("vpclattice-tg-instance"),
        "state missing target group"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_vpclattice_target_group_lambda() {
    let result = batch_apply(
        r#"
resource "aws_vpclattice_target_group" "vpclattice_tg_lambda" {
  name = "vpclattice-tg-lambda"
  type = "LAMBDA"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("vpclattice-tg-lambda"),
        "state missing target group"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_vpclattice_target_group_with_health_check() {
    let result = batch_apply(
        r#"
resource "aws_vpclattice_target_group" "vpclattice_tg_hc" {
  name = "vpclattice-tg-hc"
  type = "IP"

  config {
    port           = 8080
    protocol       = "HTTP"
    vpc_identifier = "vpc-12345678"

    health_check {
      enabled                       = true
      health_check_interval_seconds = 30
      health_check_timeout_seconds  = 5
      healthy_threshold_count       = 3
      unhealthy_threshold_count     = 3
      path                          = "/health"
      protocol                      = "HTTP"
      matcher {
        value = "200"
      }
    }
  }

  tags = {
    Name = "vpclattice-tg-hc"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("vpclattice-tg-hc"),
        "state missing target group"
    );
}

// ---------------------------------------------------------------------------
// aws_vpclattice_listener
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_vpclattice_listener_basic() {
    let result = batch_apply(
        r#"
resource "aws_vpclattice_service" "vpclattice_listener_svc" {
  name = "vpclattice-listener-svc"
}

resource "aws_vpclattice_target_group" "vpclattice_listener_tg" {
  name = "vpclattice-listener-tg"
  type = "IP"

  config {
    port           = 80
    protocol       = "HTTP"
    vpc_identifier = "vpc-12345678"
  }
}

resource "aws_vpclattice_listener" "vpclattice_listener_basic" {
  name               = "vpclattice-listener-basic"
  protocol           = "HTTP"
  service_identifier = aws_vpclattice_service.vpclattice_listener_svc.id

  default_action {
    forward {
      target_groups {
        target_group_identifier = aws_vpclattice_target_group.vpclattice_listener_tg.id
      }
    }
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("vpclattice-listener-basic"),
        "state missing listener"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_vpclattice_listener_fixed_response() {
    let result = batch_apply(
        r#"
resource "aws_vpclattice_service" "vpclattice_listener_fr_svc" {
  name = "vpclattice-listener-fr-svc"
}

resource "aws_vpclattice_listener" "vpclattice_listener_fixed_resp" {
  name               = "vpclattice-listener-fr"
  protocol           = "HTTP"
  service_identifier = aws_vpclattice_service.vpclattice_listener_fr_svc.id

  default_action {
    fixed_response {
      status_code = 404
    }
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("vpclattice-listener-fr"),
        "state missing listener"
    );
}

// ---------------------------------------------------------------------------
// aws_vpclattice_listener_rule
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_vpclattice_listener_rule_path_match() {
    let result = batch_apply(
        r#"
resource "aws_vpclattice_service" "vpclattice_rule_svc" {
  name = "vpclattice-rule-svc"
}

resource "aws_vpclattice_target_group" "vpclattice_rule_tg_default" {
  name = "vpclattice-rule-tg-def"
  type = "IP"

  config {
    port           = 80
    protocol       = "HTTP"
    vpc_identifier = "vpc-12345678"
  }
}

resource "aws_vpclattice_target_group" "vpclattice_rule_tg_api" {
  name = "vpclattice-rule-tg-api"
  type = "IP"

  config {
    port           = 8080
    protocol       = "HTTP"
    vpc_identifier = "vpc-12345678"
  }
}

resource "aws_vpclattice_listener" "vpclattice_rule_listener" {
  name               = "vpclattice-rule-listener"
  protocol           = "HTTP"
  service_identifier = aws_vpclattice_service.vpclattice_rule_svc.id

  default_action {
    forward {
      target_groups {
        target_group_identifier = aws_vpclattice_target_group.vpclattice_rule_tg_default.id
      }
    }
  }
}

resource "aws_vpclattice_listener_rule" "vpclattice_rule_path" {
  name                = "vpclattice-rule-path"
  listener_identifier = aws_vpclattice_listener.vpclattice_rule_listener.id
  service_identifier  = aws_vpclattice_service.vpclattice_rule_svc.id
  priority            = 10

  match {
    http_match {
      path_match {
        case_sensitive = false
        match {
          prefix = "/api/"
        }
      }
    }
  }

  action {
    forward {
      target_groups {
        target_group_identifier = aws_vpclattice_target_group.vpclattice_rule_tg_api.id
      }
    }
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("vpclattice-rule-path"),
        "state missing listener rule"
    );
}

// ---------------------------------------------------------------------------
// aws_vpclattice_service_network_service_association
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_vpclattice_service_network_service_association() {
    let result = batch_apply(
        r#"
resource "aws_vpclattice_service_network" "vpclattice_snsa_sn" {
  name = "vpclattice-snsa-sn"
}

resource "aws_vpclattice_service" "vpclattice_snsa_svc" {
  name = "vpclattice-snsa-svc"
}

resource "aws_vpclattice_service_network_service_association" "vpclattice_snsa" {
  service_identifier         = aws_vpclattice_service.vpclattice_snsa_svc.id
  service_network_identifier = aws_vpclattice_service_network.vpclattice_snsa_sn.id
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result
            .state
            .contains("aws_vpclattice_service_network_service_association"),
        "state missing service network service association"
    );
}

// ---------------------------------------------------------------------------
// aws_vpclattice_service_network_vpc_association
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_vpclattice_service_network_vpc_association() {
    let result = batch_apply(
        r#"
resource "aws_vpclattice_service_network" "vpclattice_snva_sn" {
  name = "vpclattice-snva-sn"
}

resource "aws_vpclattice_service_network_vpc_association" "vpclattice_snva" {
  vpc_identifier             = "vpc-12345678"
  service_network_identifier = aws_vpclattice_service_network.vpclattice_snva_sn.id
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result
            .state
            .contains("aws_vpclattice_service_network_vpc_association"),
        "state missing service network VPC association"
    );
}

// ---------------------------------------------------------------------------
// aws_vpclattice_auth_policy
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_vpclattice_auth_policy() {
    let result = batch_apply(
        r#"
resource "aws_vpclattice_service_network" "vpclattice_ap_sn" {
  name      = "vpclattice-ap-sn"
  auth_type = "AWS_IAM"
}

resource "aws_vpclattice_auth_policy" "vpclattice_ap" {
  resource_identifier = aws_vpclattice_service_network.vpclattice_ap_sn.arn
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect    = "Allow"
      Principal = "*"
      Action    = "vpc-lattice-svcs:Invoke"
      Resource  = "*"
    }]
  })
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("aws_vpclattice_auth_policy"),
        "state missing auth policy"
    );
}

// ---------------------------------------------------------------------------
// aws_vpclattice_resource_policy
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_vpclattice_resource_policy() {
    let result = batch_apply(
        r#"
resource "aws_vpclattice_service_network" "vpclattice_rp_sn" {
  name = "vpclattice-rp-sn"
}

resource "aws_vpclattice_resource_policy" "vpclattice_rp" {
  resource_arn = aws_vpclattice_service_network.vpclattice_rp_sn.arn
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect    = "Allow"
      Principal = { AWS = "arn:aws:iam::123456789012:root" }
      Action    = "vpc-lattice-svcs:*"
      Resource  = "*"
    }]
  })
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("aws_vpclattice_resource_policy"),
        "state missing resource policy"
    );
}

// ---------------------------------------------------------------------------
// Full-stack: service network + service + target group + listener + association
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_vpclattice_full_stack() {
    let result = batch_apply(
        r#"
resource "aws_vpclattice_service_network" "vpclattice_full_sn" {
  name = "vpclattice-full-sn"

  tags = {
    Environment = "e2e-test"
  }
}

resource "aws_vpclattice_service" "vpclattice_full_svc" {
  name = "vpclattice-full-svc"

  tags = {
    Environment = "e2e-test"
  }
}

resource "aws_vpclattice_target_group" "vpclattice_full_tg" {
  name = "vpclattice-full-tg"
  type = "IP"

  config {
    port           = 80
    protocol       = "HTTP"
    vpc_identifier = "vpc-12345678"
  }

  tags = {
    Environment = "e2e-test"
  }
}

resource "aws_vpclattice_listener" "vpclattice_full_listener" {
  name               = "vpclattice-full-listener"
  protocol           = "HTTP"
  service_identifier = aws_vpclattice_service.vpclattice_full_svc.id

  default_action {
    forward {
      target_groups {
        target_group_identifier = aws_vpclattice_target_group.vpclattice_full_tg.id
      }
    }
  }

  tags = {
    Environment = "e2e-test"
  }
}

resource "aws_vpclattice_service_network_service_association" "vpclattice_full_snsa" {
  service_identifier         = aws_vpclattice_service.vpclattice_full_svc.id
  service_network_identifier = aws_vpclattice_service_network.vpclattice_full_sn.id
}

resource "aws_vpclattice_service_network_vpc_association" "vpclattice_full_snva" {
  vpc_identifier             = "vpc-12345678"
  service_network_identifier = aws_vpclattice_service_network.vpclattice_full_sn.id
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("vpclattice-full-sn"),
        "state missing service network"
    );
    assert!(
        result.state.contains("vpclattice-full-svc"),
        "state missing service"
    );
    assert!(
        result.state.contains("vpclattice-full-tg"),
        "state missing target group"
    );
    assert!(
        result.state.contains("vpclattice-full-listener"),
        "state missing listener"
    );
    assert!(
        result
            .state
            .contains("aws_vpclattice_service_network_service_association"),
        "state missing service network service association"
    );
    assert!(
        result
            .state
            .contains("aws_vpclattice_service_network_vpc_association"),
        "state missing service network VPC association"
    );
}
