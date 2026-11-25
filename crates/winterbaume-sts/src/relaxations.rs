//! Relaxations of STS request validation. See [`winterbaume_core::relax`].

use winterbaume_core::Relaxation;

/// Accept `AssumeRole` without `RoleArn`, as MinIO does, and issue temporary
/// credentials for the caller's own identity. AWS answers `MissingParameter`.
///
/// Clients written against MinIO send such requests: Lakekeeper, for one,
/// vends credentials this way for `"flavor": "minio"` storage profiles.
/// `RoleSessionName` is still required. The response carries no
/// `AssumedRoleUser`, since no role is assumed, and `GetCallerIdentity` with
/// the issued credentials returns the caller's identity.
pub const ASSUME_ROLE_WITHOUT_ROLE_ARN: Relaxation = Relaxation {
    name: "sts-assume-role-without-role-arn",
    summary: "Accept AssumeRole without RoleArn (as MinIO does) and issue \
              credentials for the caller's own identity",
};

/// Every STS relaxation.
pub const RELAXATIONS: &[Relaxation] = &[ASSUME_ROLE_WITHOUT_ROLE_ARN];
