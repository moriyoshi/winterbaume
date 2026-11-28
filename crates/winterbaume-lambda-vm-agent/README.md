# winterbaume-lambda-vm-agent

The guest agent of `winterbaume-lambda-vm`. Inside a Lambda microVM it listens on guest TCP ports and relays each connection to a host vsock port. The Runtime API client and the AWS SDKs in the stock base images speak only TCP, and the host serves them on Unix sockets that libkrun connects to the vsock ports.

It is a static Linux executable without libc. System calls go through rustix's raw Linux backend, origin provides the program start-up, and connections live in a fixed table, so there is no allocator.

## Build

`winterbaume-lambda-vm`'s build script builds this crate and embeds the binary, so a server build normally carries it. This crate is outside the main workspace. To build it by hand, target the host's architecture, since guests always run on that architecture:

```sh
cd crates/winterbaume-lambda-vm-agent
rustup target add aarch64-unknown-linux-gnu   # or x86_64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
```

`.cargo/config.toml` links with `rust-lld`, so either architecture builds on any host, macOS included.

## Usage

```
winterbaume-lambda-vm-agent TCP:VSOCK [TCP:VSOCK ...]
```

The agent listens on `127.0.0.1:TCP` for every pair. Once all listeners are bound it prints `ready` and closes its stdout, then relays connections to vsock port `VSOCK` of the host (context id 2) until the VM ends. The launch script starts it as `ready=$(agent ... &)`.
