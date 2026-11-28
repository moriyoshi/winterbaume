//! Runs Lambda functions for winterbaume in libkrun microVMs.
//!
//! [`KrunExecutor`] is a [`LambdaExecutor`](winterbaume_lambda::LambdaExecutor)
//! that boots the stock AWS Lambda base image of a function's runtime (
//! `public.ecr.aws/lambda/nodejs:22`, `.../python:3.13`, ... ) in a microVM,
//! with the function's deployment package at `/var/task`, and drives the
//! image's own runtime interface client through a Lambda Runtime API that
//! this crate serves on the host.  What a function sees is therefore what it
//! sees on AWS: the same files, the same runtime, the same error documents.
//!
//! # Requirements
//!
//! - libkrun 1.x and libkrunfw ( loaded at run time, not linked ).
//! - Hardware virtualisation: `/dev/kvm` on Linux ( x86_64 or aarch64 ),
//!   Hypervisor.framework on macOS ( arm64 only; the process needs the
//!   `com.apple.security.hypervisor` entitlement ).
//! - The VM helper program ( `winterbaume-lambda-vm-helper`, or any program
//!   that calls [`helper::helper_main`] ), because libkrun turns the calling
//!   process into the VM.
//! - The guest agent ( `winterbaume-lambda-vm-agent`, a static Linux binary
//!   for the host's architecture, built from its own crate ), which relays
//!   the guest's Runtime API and AWS endpoint ports to the host over vsock.
//!
//! Whatever is missing makes `Invoke` answer with the marked stub and a
//! reason such as `libkrun-unavailable` or `vm-unavailable`.
//!
//! # Isolation
//!
//! A function cannot see the host file system: its root is the base image,
//! read-only, with a private writable `/tmp`.  It has no network either: it
//! reaches the Lambda Runtime API and, when an [`AwsEndpoint`] is connected,
//! the emulator's AWS services through the guest agent, over vsock to Unix
//! sockets in a private directory; nothing listens on a host TCP port.
//! [`KrunExecutorConfig::with_network`] gives functions the host's network
//! instead ( libkrun's transparent socket impersonation, as the user running
//! the emulator ).

pub mod cache;
pub mod config;
pub mod endpoint;
mod executor;
pub mod helper;
mod instance;
pub mod oci;
mod pool;
pub mod proto;
pub mod runtime_api;
pub mod runtimes;

pub use config::{ArchMismatch, HelperCommand, KrunExecutorConfig};
pub use endpoint::AwsEndpoint;
pub use executor::KrunExecutor;
