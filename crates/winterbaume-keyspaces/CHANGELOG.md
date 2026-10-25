# Changelog

## v0.2.1 - 2026-10-25

### Fixed

- Accept RPC-v2 CBOR clients: the server now correctly negotiates the `rpcv2-cbor` protocol, so AWS SDK clients that advertise this transport can connect without being rejected.

### Tests

- Added protocol-level test coverage for RPC-v2 CBOR client negotiation.

## v0.2.0 - 2026-05-14

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-10

### Added

- Initial public release. Amazon Keyspaces service implementation for winterbaume.
