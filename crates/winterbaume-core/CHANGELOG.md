# Changelog

## v0.3.1 - 2026-11-25

### Added
- `Relaxation`, `Relaxations` and `UnknownRelaxation` ( the `relax` module ): named, opt-in relaxations of AWS request validation. A service crate declares its rules and takes the enabled set through `with_relaxations`; with none enabled, services refuse what AWS refuses (#77, #78).
- `auth::extract_access_key_from_headers`, the access key ID from a SigV4 `Authorization` header. The signature is not checked (#78).

## v0.3.0 - 2026-11-21

### Added
- `percent_decode_path`, a decoder for URI path segments in which `+` is a literal plus, beside the existing form and query-string decoder `percent_decode` (#60).

### Fixed
- `percent_decode`, `urldecode` and `parse_query_string` decode percent-encoded UTF-8 as UTF-8. They turned each decoded byte into its own character, so every non-ASCII value in a query string or form-encoded body was mangled ( `é` as `Ã©` ) in every service that parses requests through them (#58, #60).

### Changed
- A `%` not followed by two hexadecimal digits is kept as written. It used to be dropped together with the next two bytes, so `100%` decoded to `100` (#60).

## v0.2.0 - 2026-05-13

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-09

### Added

- Initial public release. Core library for winterbaume - AWS service mocking for Rust.
