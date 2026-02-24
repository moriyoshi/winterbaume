use crate::harness::*;

// GuardDuty terraform resources tested here:
//   aws_guardduty_detector  (CreateDetector, GetDetector, UpdateDetector, DeleteDetector,
//                            ListDetectors, ListTagsForResource, TagResource, UntagResource)
//   aws_guardduty_filter    (CreateFilter, GetFilter, UpdateFilter, DeleteFilter, ListFilters)
//
// Not tested:
//   aws_guardduty_member  -- the converter and handlers store members on the
//     internal Detector struct, but the DetectorView used by the terraform
//     converter does not expose a members map. Member injection via terraform is
//     best-effort only (see crates/winterbaume-terraform/src/converters/guardduty.rs)
//     so a round-trip terraform apply does not reliably populate state, and
//     terraform's GetMembers refresh would not see the data the converter "wrote".

// ---------------------------------------------------------------------------
// aws_guardduty_detector
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_guardduty_detector_basic() {
    let result = batch_apply(
        r#"
resource "aws_guardduty_detector" "guardduty_detector_basic" {
  enable = true
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("aws_guardduty_detector"),
        "state missing detector resource"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_guardduty_detector_with_frequency() {
    let result = batch_apply(
        r#"
resource "aws_guardduty_detector" "guardduty_detector_freq" {
  enable                       = true
  finding_publishing_frequency = "FIFTEEN_MINUTES"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("FIFTEEN_MINUTES"),
        "state missing finding_publishing_frequency"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_guardduty_detector_with_tags() {
    let result = batch_apply(
        r#"
resource "aws_guardduty_detector" "guardduty_detector_tags" {
  enable = true

  tags = {
    Environment = "guardduty-e2e-test"
    Purpose     = "detector-tags"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("guardduty-e2e-test"),
        "state missing detector tag value"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_guardduty_detector_disabled() {
    let result = batch_apply(
        r#"
resource "aws_guardduty_detector" "guardduty_detector_disabled" {
  enable = false
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("aws_guardduty_detector"),
        "state missing detector resource"
    );
}

// ---------------------------------------------------------------------------
// aws_guardduty_filter
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_guardduty_filter_basic() {
    let result = batch_apply(
        r#"
resource "aws_guardduty_detector" "guardduty_filter_basic_detector" {
  enable = true
}

resource "aws_guardduty_filter" "guardduty_filter_basic" {
  name        = "guardduty-filter-basic"
  detector_id = aws_guardduty_detector.guardduty_filter_basic_detector.id
  action      = "NOOP"
  rank        = 1

  finding_criteria {
    criterion {
      field  = "severity"
      equals = ["7", "8"]
    }
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("guardduty-filter-basic"),
        "state missing filter name"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_guardduty_filter_archive_action() {
    let result = batch_apply(
        r#"
resource "aws_guardduty_detector" "guardduty_filter_archive_detector" {
  enable = true
}

resource "aws_guardduty_filter" "guardduty_filter_archive" {
  name        = "guardduty-filter-archive"
  detector_id = aws_guardduty_detector.guardduty_filter_archive_detector.id
  description = "Archive low-severity findings automatically"
  action      = "ARCHIVE"
  rank        = 2

  finding_criteria {
    criterion {
      field      = "severity"
      less_than  = "4"
    }
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("guardduty-filter-archive"),
        "state missing filter name"
    );
    assert!(
        result.state.contains("ARCHIVE"),
        "state missing filter action"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_guardduty_filter_with_tags() {
    let result = batch_apply(
        r#"
resource "aws_guardduty_detector" "guardduty_filter_tags_detector" {
  enable = true
}

resource "aws_guardduty_filter" "guardduty_filter_tags" {
  name        = "guardduty-filter-tags"
  detector_id = aws_guardduty_detector.guardduty_filter_tags_detector.id
  action      = "NOOP"
  rank        = 5

  finding_criteria {
    criterion {
      field  = "type"
      equals = ["UnauthorizedAccess:EC2/SSHBruteForce"]
    }
  }

  tags = {
    Environment = "guardduty-filter-tags-test"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("guardduty-filter-tags"),
        "state missing filter name"
    );
    assert!(
        result.state.contains("guardduty-filter-tags-test"),
        "state missing filter tag value"
    );
}

// ---------------------------------------------------------------------------
// Full-stack: detector + multiple filters
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_guardduty_full_stack() {
    let result = batch_apply(
        r#"
resource "aws_guardduty_detector" "guardduty_full_detector" {
  enable                       = true
  finding_publishing_frequency = "ONE_HOUR"

  tags = {
    Environment = "guardduty-full-stack"
  }
}

resource "aws_guardduty_filter" "guardduty_full_filter_high" {
  name        = "guardduty-full-stack-high"
  detector_id = aws_guardduty_detector.guardduty_full_detector.id
  action      = "NOOP"
  rank        = 1

  finding_criteria {
    criterion {
      field                = "severity"
      greater_than_or_equal = "7"
    }
  }
}

resource "aws_guardduty_filter" "guardduty_full_filter_low" {
  name        = "guardduty-full-stack-low"
  detector_id = aws_guardduty_detector.guardduty_full_detector.id
  description = "Auto-archive informational findings"
  action      = "ARCHIVE"
  rank        = 2

  finding_criteria {
    criterion {
      field     = "severity"
      less_than = "2"
    }
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("guardduty-full-stack"),
        "state missing detector tag value"
    );
    assert!(
        result.state.contains("guardduty-full-stack-high"),
        "state missing high-severity filter"
    );
    assert!(
        result.state.contains("guardduty-full-stack-low"),
        "state missing low-severity filter"
    );
    assert!(
        result.state.contains("ONE_HOUR"),
        "state missing finding_publishing_frequency"
    );
}
