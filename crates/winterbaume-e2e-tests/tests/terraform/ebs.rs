//! Terraform E2E coverage for the EBS service.
//!
//! NOTE — there is intentionally no terraform-driven coverage here.
//!
//! The `winterbaume-ebs` crate implements the **EBS direct API** (the
//! block-level snapshot data plane: `StartSnapshot`, `CompleteSnapshot`,
//! `PutSnapshotBlock`, `GetSnapshotBlock`, `ListSnapshotBlocks`,
//! `ListChangedBlocks`). Those routes live under
//! `https://ebs.<region>.amazonaws.com/snapshots/...` and are consumed by
//! the AWS SDK directly (e.g., for block-level snapshot copy or restore);
//! the terraform AWS provider does **not** expose any resource that calls
//! these operations.
//!
//! The terraform resources one might expect to live here, namely
//! `aws_ebs_volume` and `aws_ebs_snapshot`, are actually dispatched by the
//! provider to the **EC2 service endpoint** (`CreateVolume`,
//! `DescribeVolumes`, `CreateSnapshot`, `DescribeSnapshots`, ...). Those
//! handlers already live in `winterbaume-ec2` and are exercised by
//! `tests/terraform/ec2.rs`. Likewise the terraform AWS provider has no
//! `ebs` entry in its endpoint resolver — there is nothing for an `ebs`
//! key in `write_provider_tf` to point at.
//!
//! Consequently:
//!
//! * The `EbsService` does **not** need to be registered in
//!   `harness::test_services()` for terraform coverage.
//! * `write_provider_tf` does **not** need an `ebs = "..."` entry.
//! * No terraform `apply` invocation in this module is meaningful.
//!
//! The single placeholder test below documents this situation in code so
//! that future agents do not repeatedly try to add `aws_ebs_volume` cases
//! here (they belong in `ec2.rs`) and so that `cargo test -- --ignored`
//! never executes a real terraform run for this module.

#[allow(unused_imports)]
use crate::harness::*;

/// Placeholder — see the module-level comment for why there are no real
/// EBS terraform tests. Always `#[ignore]`d, never actually exercised by
/// `cargo test -- --ignored` in any meaningful way.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "EBS direct API has no terraform-provider surface; volume/snapshot resources are tested via ec2.rs"]
async fn test_ebs_no_terraform_surface_placeholder() {
    // Intentionally empty: the EBS direct API (snapshot blocks /
    // changed-blocks) is not reachable through any terraform AWS provider
    // resource. `aws_ebs_volume` and `aws_ebs_snapshot` are EC2 endpoint
    // operations and are covered in `tests/terraform/ec2.rs`.
}
