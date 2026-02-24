use crate::harness::*;

// WAFv2 terraform resources tested here:
//   aws_wafv2_web_acl         (CreateWebACL, GetWebACL, UpdateWebACL, DeleteWebACL,
//                              ListWebACLs, ListTagsForResource, TagResource)
//   aws_wafv2_ip_set          (CreateIPSet, GetIPSet, UpdateIPSet, DeleteIPSet,
//                              ListIPSets, ListTagsForResource)
//   aws_wafv2_regex_pattern_set (CreateRegexPatternSet, GetRegexPatternSet, UpdateRegexPatternSet,
//                                DeleteRegexPatternSet, ListRegexPatternSets)
//   aws_wafv2_rule_group      (CreateRuleGroup, GetRuleGroup, UpdateRuleGroup, DeleteRuleGroup,
//                              ListRuleGroups)
//
// All tests use scope = "REGIONAL" (us-east-1 region) unless noted otherwise.
//
// Not tested:
//   aws_wafv2_web_acl_logging_configuration — requires PutLoggingConfiguration with a
//     real Kinesis/CWL ARN; too complex for mock testing.
//   aws_wafv2_web_acl_association           — requires ListResourcesForWebACL (not implemented).

// ---------------------------------------------------------------------------
// aws_wafv2_web_acl
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_wafv2_web_acl_basic() {
    let result = batch_apply(
        r#"
resource "aws_wafv2_web_acl" "wafv2_web_acl_basic" {
  name  = "wafv2-web-acl-basic"
  scope = "REGIONAL"

  default_action {
    allow {}
  }

  visibility_config {
    cloudwatch_metrics_enabled = false
    metric_name                = "wafv2-web-acl-basic"
    sampled_requests_enabled   = false
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("wafv2-web-acl-basic"),
        "state missing web ACL name"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_wafv2_web_acl_with_block_default() {
    let result = batch_apply(
        r#"
resource "aws_wafv2_web_acl" "wafv2_web_acl_block" {
  name        = "wafv2-web-acl-block"
  scope       = "REGIONAL"
  description = "WAF ACL with block default action"

  default_action {
    block {}
  }

  visibility_config {
    cloudwatch_metrics_enabled = false
    metric_name                = "wafv2-web-acl-block"
    sampled_requests_enabled   = false
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("wafv2-web-acl-block"),
        "state missing web ACL name"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_wafv2_web_acl_with_tags() {
    let result = batch_apply(
        r#"
resource "aws_wafv2_web_acl" "wafv2_web_acl_tags" {
  name  = "wafv2-web-acl-tags"
  scope = "REGIONAL"

  default_action {
    allow {}
  }

  visibility_config {
    cloudwatch_metrics_enabled = false
    metric_name                = "wafv2-web-acl-tags"
    sampled_requests_enabled   = false
  }

  tags = {
    Environment = "test"
    Service     = "wafv2"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("wafv2-web-acl-tags"),
        "state missing web ACL name"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_wafv2_web_acl_with_ip_rate_rule() {
    let result = batch_apply(
        r#"
resource "aws_wafv2_ip_set" "wafv2_web_acl_ip_rate_allow_list" {
  name               = "wafv2-web-acl-ip-rate-allow-list"
  scope              = "REGIONAL"
  ip_address_version = "IPV4"
  addresses          = ["203.0.113.0/24"]
}

resource "aws_wafv2_web_acl" "wafv2_web_acl_ip_rate_rule" {
  name  = "wafv2-web-acl-ip-rate-rule"
  scope = "REGIONAL"

  default_action {
    allow {}
  }

  rule {
    name     = "ip-rate-limit"
    priority = 1

    action {
      block {}
    }

    statement {
      ip_set_reference_statement {
        arn = aws_wafv2_ip_set.wafv2_web_acl_ip_rate_allow_list.arn
      }
    }

    visibility_config {
      cloudwatch_metrics_enabled = false
      metric_name                = "ip-rate-limit"
      sampled_requests_enabled   = false
    }
  }

  visibility_config {
    cloudwatch_metrics_enabled = false
    metric_name                = "wafv2-web-acl-ip-rate-rule"
    sampled_requests_enabled   = false
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("wafv2-web-acl-ip-rate-rule"),
        "state missing web ACL name"
    );
}

// ---------------------------------------------------------------------------
// aws_wafv2_ip_set
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_wafv2_ip_set_basic() {
    let result = batch_apply(
        r#"
resource "aws_wafv2_ip_set" "wafv2_ip_set_basic" {
  name               = "wafv2-ip-set-basic"
  scope              = "REGIONAL"
  ip_address_version = "IPV4"
  addresses          = ["192.0.2.0/24", "198.51.100.0/24"]
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("wafv2-ip-set-basic"),
        "state missing IP set name"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_wafv2_ip_set_ipv6() {
    let result = batch_apply(
        r#"
resource "aws_wafv2_ip_set" "wafv2_ip_set_ipv6" {
  name               = "wafv2-ip-set-ipv6"
  scope              = "REGIONAL"
  ip_address_version = "IPV6"
  addresses          = ["2001:db8::/32"]
  description        = "IPv6 block list"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("wafv2-ip-set-ipv6"),
        "state missing IP set name"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_wafv2_ip_set_with_tags() {
    let result = batch_apply(
        r#"
resource "aws_wafv2_ip_set" "wafv2_ip_set_tags" {
  name               = "wafv2-ip-set-tags"
  scope              = "REGIONAL"
  ip_address_version = "IPV4"
  addresses          = ["10.0.0.0/8"]

  tags = {
    Environment = "test"
    Purpose     = "blocklist"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("wafv2-ip-set-tags"),
        "state missing IP set"
    );
}

// ---------------------------------------------------------------------------
// aws_wafv2_regex_pattern_set
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_wafv2_regex_pattern_set_basic() {
    let result = batch_apply(
        r#"
resource "aws_wafv2_regex_pattern_set" "wafv2_regex_pattern_set_basic" {
  name  = "wafv2-regex-pattern-set-basic"
  scope = "REGIONAL"

  regular_expression {
    regex_string = "^/admin/.*"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("wafv2-regex-pattern-set-basic"),
        "state missing regex pattern set"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_wafv2_regex_pattern_set_multiple_patterns() {
    let result = batch_apply(
        r#"
resource "aws_wafv2_regex_pattern_set" "wafv2_regex_pattern_set_multi" {
  name        = "wafv2-regex-pattern-set-multi"
  scope       = "REGIONAL"
  description = "Multiple SQL injection patterns"

  regular_expression {
    regex_string = "(?i)select.*from"
  }

  regular_expression {
    regex_string = "(?i)union.*select"
  }

  regular_expression {
    regex_string = "(?i)insert.*into"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("wafv2-regex-pattern-set-multi"),
        "state missing regex pattern set"
    );
}

// ---------------------------------------------------------------------------
// aws_wafv2_rule_group
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_wafv2_rule_group_basic() {
    let result = batch_apply(
        r#"
resource "aws_wafv2_rule_group" "wafv2_rule_group_basic" {
  name     = "wafv2-rule-group-basic"
  scope    = "REGIONAL"
  capacity = 10

  visibility_config {
    cloudwatch_metrics_enabled = false
    metric_name                = "wafv2-rule-group-basic"
    sampled_requests_enabled   = false
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("wafv2-rule-group-basic"),
        "state missing rule group name"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_wafv2_rule_group_with_tags() {
    let result = batch_apply(
        r#"
resource "aws_wafv2_rule_group" "wafv2_rule_group_tags" {
  name        = "wafv2-rule-group-tags"
  scope       = "REGIONAL"
  capacity    = 20
  description = "Rule group with tags"

  visibility_config {
    cloudwatch_metrics_enabled = false
    metric_name                = "wafv2-rule-group-tags"
    sampled_requests_enabled   = false
  }

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("wafv2-rule-group-tags"),
        "state missing rule group"
    );
}

// ---------------------------------------------------------------------------
// Full-stack: web ACL + IP set + regex pattern set + rule group
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_wafv2_full_stack() {
    let result = batch_apply(
        r#"
resource "aws_wafv2_ip_set" "wafv2_full_ip_set" {
  name               = "wafv2-full-stack-ip-set"
  scope              = "REGIONAL"
  ip_address_version = "IPV4"
  addresses          = ["198.51.100.0/24"]
}

resource "aws_wafv2_regex_pattern_set" "wafv2_full_regex" {
  name  = "wafv2-full-stack-regex"
  scope = "REGIONAL"

  regular_expression {
    regex_string = "^/api/v[0-9]+/.*"
  }
}

resource "aws_wafv2_rule_group" "wafv2_full_rule_group" {
  name     = "wafv2-full-stack-rule-group"
  scope    = "REGIONAL"
  capacity = 50

  visibility_config {
    cloudwatch_metrics_enabled = false
    metric_name                = "wafv2-full-stack-rule-group"
    sampled_requests_enabled   = false
  }
}

resource "aws_wafv2_web_acl" "wafv2_full_web_acl" {
  name  = "wafv2-full-stack-web-acl"
  scope = "REGIONAL"

  default_action {
    allow {}
  }

  rule {
    name     = "block-bad-ips"
    priority = 1

    action {
      block {}
    }

    statement {
      ip_set_reference_statement {
        arn = aws_wafv2_ip_set.wafv2_full_ip_set.arn
      }
    }

    visibility_config {
      cloudwatch_metrics_enabled = false
      metric_name                = "block-bad-ips"
      sampled_requests_enabled   = false
    }
  }

  visibility_config {
    cloudwatch_metrics_enabled = false
    metric_name                = "wafv2-full-stack-web-acl"
    sampled_requests_enabled   = false
  }

  tags = {
    Environment = "e2e-test"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("wafv2-full-stack-ip-set"),
        "state missing IP set"
    );
    assert!(
        result.state.contains("wafv2-full-stack-regex"),
        "state missing regex pattern set"
    );
    assert!(
        result.state.contains("wafv2-full-stack-rule-group"),
        "state missing rule group"
    );
    assert!(
        result.state.contains("wafv2-full-stack-web-acl"),
        "state missing web ACL"
    );
}
