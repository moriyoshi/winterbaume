//! Relaxations of Lambda behaviour. See [`winterbaume_core::relax`].

use winterbaume_core::Relaxation;

/// Run a function whose `Architectures` differs from the host's on the
/// host's instruction set, with the base image for that architecture,
/// instead of answering `Invoke` with the `architecture-unsupported` stub.
///
/// An executor cannot emulate another instruction set, and AWS defaults
/// `Architectures` to `x86_64`, so on an arm64 host ( Apple silicon, Graviton )
/// most functions would otherwise not run.  Interpreted code usually does not
/// care; native modules built for the configured architecture fail to load.
/// Responses of substituted runs carry
/// `x-winterbaume-lambda-architecture-substituted`.
pub const RUN_ON_HOST_ARCHITECTURE: Relaxation = Relaxation {
    name: "lambda-run-on-host-architecture",
    summary: "Run Lambda functions configured for another architecture (x86_64 on an arm64 \
              host, or the reverse) on the host's instead of answering Invoke with a stub",
};

/// Every Lambda relaxation.
pub const RELAXATIONS: &[Relaxation] = &[RUN_ON_HOST_ARCHITECTURE];
