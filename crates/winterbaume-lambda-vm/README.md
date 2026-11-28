# winterbaume-lambda-vm

Runs Lambda functions for [winterbaume](https://github.com/moriyoshi/winterbaume) in [libkrun](https://github.com/containers/libkrun) microVMs, booted from the stock AWS Lambda base images.

`KrunExecutor` is a `winterbaume_lambda::LambdaExecutor`. Plugged into `winterbaume_lambda::LambdaService` with `with_code_execution`, it makes `Invoke` run the deployed ZIP package. Each execution environment is a microVM booted from the base image of the function's runtime ( `public.ecr.aws/lambda/nodejs:22`, `.../python:3.13`, `.../provided:al2023`, ... ), with the package at `/var/task`. The image's own runtime interface client runs the handler, so handler loading, the `context` object and error documents match AWS.

```rust,no_run
use std::sync::Arc;
use winterbaume_lambda::LambdaService;
use winterbaume_lambda_vm::{KrunExecutor, KrunExecutorConfig};

let executor = KrunExecutor::new(KrunExecutorConfig::new());
let lambda = LambdaService::new().with_code_execution(Arc::new(executor));
# let _ = lambda;
```

## Requirements

- **libkrun 1.x and libkrunfw.** They are loaded at run time, not linked, so building this crate needs neither. Use `KrunExecutorConfig::with_libkrun` when the dynamic loader does not find libkrun.
- **Hardware virtualisation.** On Linux ( x86_64 or aarch64 ) you need `/dev/kvm`. On macOS ( arm64 only ) you need Hypervisor.framework, and the helper binary must carry the `com.apple.security.hypervisor` entitlement.
- **A VM helper program.** libkrun turns the calling process into the VM, so every VM is a helper process. Use the `winterbaume-lambda-vm-helper` binary of this crate, or any program that calls `winterbaume_lambda_vm::helper::helper_main()`. The executor finds the helper through `KrunExecutorConfig::with_helper`, the `WB_LAMBDA_VM_HELPER` variable, next to the running executable, or on `PATH`.
- **The guest agent.** This is a static, libc-free Linux binary of about 6 KB, from `crates/winterbaume-lambda-vm-agent`.
  - In a repository checkout, this crate's build script builds the agent for the host's architecture and embeds it. This needs the Rust target for the guest: the host's own on Linux, `aarch64-unknown-linux-gnu` on macOS.
  - `WINTERBAUME_LAMBDA_VM_AGENT` embeds a prebuilt agent instead.
  - `WINTERBAUME_LAMBDA_VM_REQUIRE_AGENT=1` makes a missing agent a build error rather than a warning.
  - At run time the executor uses `KrunExecutorConfig::with_agent`, then the `WB_LAMBDA_VM_AGENT` variable, then `winterbaume-lambda-vm-agent` next to the helper or the running executable. It falls back to the embedded copy, which it writes once into its cache directory.
- **Base images.** These come from `public.ecr.aws` ( pulled anonymously on first use and cached on disk; the cache needs a case-sensitive file system ), or from an image directory given with `KrunExecutorConfig::with_image_dir`.

Whatever is missing makes `Invoke` answer with the marked stub and a reason such as `libkrun-unavailable`, `vm-unavailable` or `image-unavailable`.

## Behaviour

- **Warm reuse.** Execution environments are reused for the same function version and configuration until they have been idle for the warm TTL ( 300 s by default ). An environment that failed or timed out is discarded, and so are all environments of a function that was updated or deleted.
- **File system.** The root is read-only. `/tmp` is the guest's tmpfs, which is not executable.
- **Network.** By default the guest has none. The guest agent relays two guest ports over vsock to Unix sockets in a private host directory: the Lambda Runtime API on `127.0.0.1:9001`, and, when an `AwsEndpoint` is connected, the emulator's AWS services on `127.0.0.1:4566`, which becomes `AWS_ENDPOINT_URL`. Nothing listens on a host TCP port. `KrunExecutorConfig::with_network(true)` gives the guest the host's network instead, as the user running the emulator.
- **Architecture.** A function whose `Architectures` differs from the host's is not run unless the `lambda-run-on-host-architecture` relaxation is enabled.

## Testing

- `cargo test --features fake-helper` runs the executor end to end with `winterbaume-lambda-vm-fake-helper`. That program plays a Lambda runtime on the host instead of booting a VM, so these tests need no hypervisor. Their image caches are in temporary directories, so on macOS `TMPDIR` must be on a case-sensitive volume.
- `tests/vm_test.rs` runs real functions in real VMs when `WB_LAMBDA_VM_TEST=1`. See the file header for the variables it reads.
