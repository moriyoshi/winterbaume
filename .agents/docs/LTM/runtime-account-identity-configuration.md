# Runtime Account Identity Configuration

## Summary

`winterbaume-server --account-id`, `WB_ACCOUNT_ID`, and the TOML `account_id` setting now affect runtime handler behaviour, not only Terraform state injection. The fix made the previously hard-coded default account ID a process-wide configurable accessor, migrated service handlers to call that accessor, and installed the configured account before service registration. This preserves the existing single-global-account semantics while making the documented configuration path observable in responses, ARNs, and per-account state lookup.

## Key Facts

- The documented precedence is `CLI > env > config > default`.
- Before the fix, `ServerOptions.account_id` only fed Terraform `ConversionContext.default_account_id`.
- Service handlers used `winterbaume_core::state::DEFAULT_ACCOUNT_ID = "123456789012"` directly for ARNs, `Owner` / `AccountId` fields, and `BackendState::get(account_id, region)`.
- `winterbaume_core::default_account_id()` is now the runtime accessor.
- `winterbaume_core::set_default_account_id(...)` installs a process-wide account ID through `OnceLock`.
- First writer wins. Calling with the literal default is a no-op.
- `DEFAULT_ACCOUNT_ID` remains exported for tests and explicit comparisons against the literal default.
- `winterbaume-server` calls `set_default_account_id(opts.account_id.clone())` before `register_all_services`.
- `MockAws::builder().account_id(...)` still affects the `mock.account_id()` getter only; it does not install the process-wide runtime override.

## Details

### Symptom

Starting `winterbaume-server` with a non-default account ID still produced default-account responses:

- S3 `list-buckets` returned `Owner.ID = "123456789012"`.
- STS `GetCallerIdentity` returned `Account = "123456789012"`.
- Service-generated ARNs embedded `123456789012`.

The configuration was parsed correctly but not consumed by handlers.

### Root Cause

`crates/winterbaume-server/src/main.rs::parse_options` merged account ID options correctly into `ServerOptions.account_id`. The only consumer was `load_tfstate`, where the value became `ConversionContext.default_account_id`.

Every runtime service handler still got the effective account by referencing the hard-coded constant. The constant appeared across hundreds of service source files and drove:

- ARN construction
- account and owner response fields
- per-account state isolation keys

No request path threaded the configured account ID to the handlers.

### Fix Shape

`crates/winterbaume-core/src/state.rs` now owns:

```rust
static CONFIGURED_ACCOUNT_ID: OnceLock<&'static str> = OnceLock::new();
pub fn set_default_account_id(id: impl Into<String>);
pub fn default_account_id() -> &'static str;
```

`set_default_account_id` leaks the supplied `String` to obtain a `&'static str` suitable for `OnceLock`. The first configured value wins; later attempts are ignored. Passing `"123456789012"` does not initialise the lock, so explicit defaults do not block a later non-default in the same process before installation.

The core crate re-exports both functions. Service code was mechanically migrated:

- imported `DEFAULT_ACCOUNT_ID` became `default_account_id`
- non-import `DEFAULT_ACCOUNT_ID` tokens became `default_account_id()`
- generated core files that define or intentionally use the constant were excluded

Two Rust format-string interpolation sites needed manual conversion because `{DEFAULT_ACCOUNT_ID}` requires a bare identifier and cannot call `default_account_id()` inside braces:

- `crates/winterbaume-sts/src/state.rs`
- `crates/winterbaume-mediastore/src/state.rs`

`crates/winterbaume-codepipeline` also had a private `DEFAULT_ACCOUNT_ID_STR`; it was removed and replaced with `winterbaume_core::default_account_id()`.

### Why Process-Wide

The cleaner architectural alternative would be a request or server context such as `Arc<str>` or `ServerContext { account_id, default_region }` threaded through every service constructor and handler.

That was rejected for this fix because:

- the `MockService` trait has 200+ implementors
- handler signatures and dispatch wiring would all change
- the existing behaviour was already a single process-wide account ID through a constant

Making that global configurable is therefore consistent with the current architecture and much smaller in blast radius.

The limitation is real: two `MockAws` instances in the same process cannot have different runtime account IDs. That was already true for handlers before this fix because `MockAws::builder().account_id(...)` did not affect service handlers.

## Files

- `crates/winterbaume-core/src/state.rs`: `CONFIGURED_ACCOUNT_ID`, `set_default_account_id`, `default_account_id`, and retained `DEFAULT_ACCOUNT_ID`.
- `crates/winterbaume-core/src/lib.rs`: re-exports.
- `crates/winterbaume-core/tests/default_account_id.rs`: regression test.
- `crates/winterbaume-server/src/main.rs`: startup installation and updated `--account-id` help.
- `crates/winterbaume-*/src/**/*.rs`: handler and state migrations from `DEFAULT_ACCOUNT_ID` to `default_account_id()`.
- `crates/winterbaume-codepipeline/src/state.rs` and `handlers.rs`: removed private parallel default-account constant.

## Test Coverage

Regression test:

```bash
./.agents/bin/cargo.sh test -p winterbaume-core --test default_account_id
```

The migration was also verified with:

```bash
./.agents/bin/cargo.sh check --workspace --all-targets
./.agents/bin/cargo.sh fmt --all -- --check
./.agents/bin/cargo.sh clippy -p winterbaume-core --all-targets --all-features -- -D warnings
./.agents/bin/cargo.sh clippy -p winterbaume-server --all-targets --all-features -- -D warnings
./.agents/bin/cargo.sh clippy -p winterbaume-s3 --all-targets --all-features -- -D warnings
./.agents/bin/cargo.sh clippy -p winterbaume-codepipeline --all-targets --all-features -- -D warnings
./.agents/bin/cargo.sh clippy -p winterbaume-sts --all-targets --all-features -- -D warnings
./.agents/bin/cargo.sh clippy -p winterbaume-mediastore --all-targets --all-features -- -D warnings
```

## Pitfalls

- Do not introduce new runtime handler references to `DEFAULT_ACCOUNT_ID`; use `default_account_id()`.
- Do not assume `MockAws::builder().account_id(...)` changes handler runtime account ID.
- Do not interpolate `default_account_id()` directly inside Rust format-string braces; pass it as a positional or named argument.
- Do not call `set_default_account_id` in tests that share a process with tests expecting the unconfigured fallback. Integration-test binaries are useful because each `tests/*.rs` file is a separate process.
- Do not treat the process-wide accessor as a permanent multi-tenant design. It is a pragmatic fix for the current single-account runtime model.

