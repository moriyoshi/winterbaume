//! Regression test for the runtime-configurable default account ID.
//!
//! Each `tests/*.rs` file gets its own test binary, so the `OnceLock`-backed
//! global lives in a fresh process. That keeps this test independent of the
//! crate's unit tests, which observe the unconfigured fallback.

use winterbaume_core::{DEFAULT_ACCOUNT_ID, default_account_id, set_default_account_id};

#[test]
fn set_then_get_returns_configured_value() {
    assert_eq!(default_account_id(), DEFAULT_ACCOUNT_ID);

    set_default_account_id("999988887777");
    assert_eq!(default_account_id(), "999988887777");

    // First-writer-wins: a second call with a different value is a no-op.
    set_default_account_id("111122223333");
    assert_eq!(default_account_id(), "999988887777");
}
