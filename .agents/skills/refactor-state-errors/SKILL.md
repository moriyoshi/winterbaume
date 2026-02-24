---
name: refactor-state-errors
description: Refactor a service crate's state module to use domain-specific error enums instead of constructing end-user-facing error structs (error_type/message/status). Moves error shaping responsibility into the handler layer.
argument-hint: <service-name>
user_invocable: true
---

# Refactor State Errors

Replace the `{Service}Error { error_type, message, status }` anti-pattern in a service crate's `state.rs` with a domain-specific error enum. Move the mapping from domain errors to wire error responses into `handlers.rs`, restoring the separation of concerns described in ARCHITECTURE.md.

## Arguments

- `$0` — Service name matching the winterbaume crate suffix (e.g., `ses`, `guardduty`, `ssm`, `lambda`)

---

## Background

### The anti-pattern (current state)

Nearly every service crate defines this in `state.rs`:

```rust
pub struct SesError {
    pub error_type: String,   // AWS error code, e.g. "NotFoundException"
    pub message: String,      // Human-readable message
    pub status: u16,          // HTTP status code — does NOT belong in state
}
```

State methods construct these structs directly, coupling business logic to HTTP semantics:

```rust
pub fn get_email_identity(&self, name: &str) -> Result<&EmailIdentity, SesError> {
    self.identities.get(name).ok_or_else(|| SesError {
        error_type: "NotFoundException".to_string(),
        message: format!("Identity {name} does not exist."),
        status: 404,
    })
}
```

The handler then has a trivial pass-through function like `ses_error_response(err: &SesError) -> MockResponse` that just copies the three fields into the wire format.

### The target pattern

**`state.rs`** — domain-specific error enum using `thiserror`, with no HTTP knowledge:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SesError {
    #[error("Identity {name} does not exist.")]
    IdentityNotFound { name: String },
    #[error("Identity {name} already exists.")]
    IdentityAlreadyExists { name: String },
    #[error("ConfigurationSet {name} does not exist.")]
    ConfigurationSetNotFound { name: String },
    #[error("ConfigurationSet {name} already exists.")]
    ConfigurationSetAlreadyExists { name: String },
    #[error("Contact list {name} does not exist.")]
    ContactListNotFound { name: String },
    // ... more variants as needed
}
```

The `#[error("...")]` attribute generates `Display` with the message template. The message text should match the original message produced by the old code.

**Important:** Do NOT add `error_type()`, `status()`, or any other method on the error enum that returns HTTP status codes or AWS error type strings. These belong exclusively in the handler's error-shaping function. The whole point of this refactoring is to remove HTTP/wire knowledge from `state.rs`.

**`handlers.rs`** — error shaping maps variants to `(status, error_type)` and uses `err.to_string()` for the message:

```rust
fn ses_error_response(err: &SesError) -> MockResponse {
    let (status, error_type) = match err {
        SesError::IdentityNotFound { .. } => (404, "NotFoundException"),
        SesError::IdentityAlreadyExists { .. } => (400, "AlreadyExistsException"),
        SesError::ConfigurationSetNotFound { .. } => (404, "NotFoundException"),
        // ...
    };
    rest_json_error(status, error_type, &err.to_string())
}
```

---

## Step 0: Read the current code

Read all four source files to understand the service's error landscape:

```
crates/winterbaume-{service}/src/state.rs
crates/winterbaume-{service}/src/handlers.rs
crates/winterbaume-{service}/src/types.rs
crates/winterbaume-{service}/src/lib.rs
```

### 0a. Inventory all error constructions in state.rs

For every `{Service}Error { ... }` construction in `state.rs`, record:

| # | Method | error_type | message template | status | Proposed enum variant |
|---|--------|------------|------------------|--------|-----------------------|
| 1 | `get_email_identity` | `NotFoundException` | `"Identity {name} does not exist."` | 404 | `IdentityNotFound { name }` |
| 2 | `create_email_identity` | `AlreadyExistsException` | `"Identity {name} already exists."` | 400 | `IdentityAlreadyExists { name }` |
| ... | ... | ... | ... | ... | ... |

### 0b. Check for error helper functions in state.rs

Some services define helper methods like `fn detector_not_found() -> {Service}Error`. These should be replaced by enum variant constructors.

### 0c. Check for error constructions in handlers.rs

Handlers may also construct `{Service}Error` structs directly (e.g., for input validation in the handler before calling state). Those should either:
- Stay as direct `rest_json_error()` / `error_response()` calls (for pure request-parsing errors), or
- Be converted to enum variants if they represent domain logic.

### 0d. Check for external usage

```bash
grep -rn '{Service}Error' crates/winterbaume-{service}/src/
```

Confirm `{Service}Error` is only used in `state.rs` and `handlers.rs`. If `views.rs` or `lib.rs` reference it, plan accordingly.

---

## Step 1: Design the error enum

### Naming guidelines

- **Group by semantics, not by AWS error code.** Multiple AWS error codes that mean the same thing at the domain level (e.g., `NotFoundException` for identities vs configuration sets) should be separate variants if they carry different data, but can share a variant if they're truly identical.
- **Carry the minimum data needed to reconstruct the message.** Usually just a name/ID string.
- **Use descriptive variant names.** Prefer `IdentityNotFound` over `NotFound` when the service has multiple resource types. For services with a single primary resource type, `NotFound { id }` is fine.
- **Each variant gets an `#[error("...")]` attribute** that produces the exact message the old code generated. Use `thiserror`'s interpolation: `#[error("Identity {name} does not exist.")]`.
- **Include a generic `Validation` variant** for validation errors that don't fit a specific category:
  ```rust
  #[error("{message}")]
  Validation { message: String },
  ```
  This covers ad-hoc validation checks where defining a specific variant would be excessive.

### Deduplication

If the same `(error_type, status)` pair is used for multiple resources with the same message template (just different resource names), consider:
- Separate variants if the resource type is clear from context (preferred for clarity)
- A parameterised variant like `#[error("{resource_type} {name} does not exist.")] NotFound { resource_type: &'static str, name: String }` if there are many (5+) resource types with identical semantics

---

## Step 2: Implement the error enum in state.rs

### 2a. Add `thiserror` to the crate's `Cargo.toml`

`thiserror` is already a workspace dependency. Add it to the service crate's `[dependencies]`:

```toml
thiserror = { workspace = true }
```

### 2b. Replace the struct with an enum

Remove:

```rust
#[derive(Debug)]
pub struct {Service}Error {
    pub error_type: String,
    pub message: String,
    pub status: u16,
}
```

Add:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum {Service}Error {
    #[error("Identity {name} does not exist.")]
    IdentityNotFound { name: String },
    #[error("Identity {name} already exists.")]
    IdentityAlreadyExists { name: String },
    // ... all variants from the inventory
}
```

Each `#[error("...")]` attribute must produce the **exact same message** as the old `format!(...)` / string literal in the original `{Service}Error` struct construction.

### 2c. Update all error constructions

Replace every `{Service}Error { error_type: ..., message: ..., status: ... }` with the appropriate enum variant.

**Before:**
```rust
Err(SesError {
    error_type: "NotFoundException".to_string(),
    message: format!("Identity {name} does not exist."),
    status: 404,
})
```

**After:**
```rust
Err(SesError::IdentityNotFound { name: name.to_string() })
```

### 2d. Remove error helper methods

Replace helper methods like `fn detector_not_found() -> {Service}Error` with direct enum variant usage. The enum variant is already as concise as a helper.

### 2e. Handle the `Validation` variant

For ad-hoc validation errors that construct messages inline, use the `Validation` variant:

**Before:**
```rust
return Err(SesError {
    error_type: "BadRequestException".to_string(),
    message: format!("Invalid email address: {addr}"),
    status: 400,
});
```

**After:**
```rust
return Err(SesError::Validation {
    message: format!("Invalid email address: {addr}"),
});
```

Only use `Validation` as a catch-all. If a specific validation failure is common enough to warrant its own variant (appears 3+ times), give it one.

---

## Step 3: Update the handler error-shaping function

### 3a. Replace the pass-through function with a match

**Before (pass-through):**
```rust
fn ses_error_response(err: &SesError) -> MockResponse {
    let body = json!({ "Type": "User", "Message": err.message });
    let mut resp = MockResponse::rest_json(err.status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, err.error_type.parse().unwrap());
    resp
}
```

**After (error shaping):**
```rust
fn ses_error_response(err: &SesError) -> MockResponse {
    let (status, error_type) = match err {
        SesError::IdentityNotFound { .. } => (404, "NotFoundException"),
        SesError::IdentityAlreadyExists { .. } => (400, "AlreadyExistsException"),
        SesError::ConfigurationSetNotFound { .. } => (404, "NotFoundException"),
        SesError::Validation { .. } => (400, "BadRequestException"),
        // ... all variants — NO wildcard arm
    };
    // Message comes from thiserror's Display impl via .to_string()
    rest_json_error(status, error_type, &err.to_string())
}
```

### 3b. Use `err.to_string()` for the message — do NOT duplicate message text

The message in the error response MUST come from `err.to_string()`, which delegates to the `#[error("...")]` attribute on the enum variant. Do NOT construct message strings inline in the match arms — this duplicates the message text and creates a maintenance risk. The match arms should only determine `(status, error_type)`.

**Wrong** (message duplicated in two places):
```rust
TimestreamWriteError::DatabaseNotFound { database_name } => (
    "ResourceNotFoundException", 404,
    format!("The database {database_name} does not exist."),  // BAD: duplicates #[error]
),
```

**Right** (single source of truth):
```rust
let (status, error_type) = match err {
    TimestreamWriteError::DatabaseNotFound { .. } => (404, "ResourceNotFoundException"),
    // ...
};
json_error_response(status, error_type, &err.to_string())  // message from #[error]
```

### 3c. Protocol-specific response format

The response format depends on the service's protocol. Use the existing `rest_json_error` / `error_response` helper that the handler already has.

| Protocol | Error body format | Header |
|----------|------------------|--------|
| restJson1 | `{"Type": "User", "Message": "..."}` | `x-amzn-errortype: ErrorCode` |
| awsJson1.x | `{"__type": "ErrorCode", "message": "..."}` | (none) |
| awsQuery | XML `<ErrorResponse><Error><Code>...</Code><Message>...</Message></Error></ErrorResponse>` | (none) |
| restXml | XML error body | (none) |

### 3d. Keep the match exhaustive

The match in the error-shaping function must be exhaustive (no wildcard `_` arm). This ensures that adding a new error variant in `state.rs` forces an update to the handler's error shaping — a compile-time guarantee that all domain errors have a defined wire representation.

---

## Step 4: Handle handler-originated errors

Some handlers construct `{Service}Error` for input-validation errors before calling state methods. Decide for each:

1. **Pure request-parsing errors** (missing JSON field, invalid JSON, unknown route) — keep as direct `rest_json_error()` calls; these are not domain errors.
2. **Domain validation errors** (invalid resource name format, conflicting parameters) — convert to enum variants and route through state, or use `{Service}Error::Validation { message }` at the handler level.

If the handler constructs `{Service}Error` structs directly (not calling state), update those to use enum variants too.

---

## Step 5: Update lib.rs exports

If `{Service}Error` is `pub` in `lib.rs` (for use in tests or views), ensure the enum is still exported. The type name stays the same, so this is usually a no-op.

---

## Step 6: Build and test

```bash
cargo build -p winterbaume-{service}
cargo clippy -p winterbaume-{service}
cargo test -p winterbaume-{service}
```

### Common issues

- **Non-exhaustive match**: add missing arms to the error-shaping function.
- **Field access on enum**: code that previously did `err.error_type` or `err.status` must use the match-based error-shaping function instead.
- **Type mismatch in tests**: if tests construct `{Service}Error` directly, update them to use enum variants.

### Verify correctness

Run the integration tests. The wire-level behaviour must not change — the same HTTP status codes, error type strings, and messages must be produced for the same inputs. This is a pure refactor.

```bash
cargo test -p winterbaume-{service} -- --test-threads=1
```

---

## Step 7: Verify no regressions

Compare the error-shaping match arms against the original inventory (Step 0a). Every `(error_type, message template, status)` triple must appear in the new match. Missing entries mean the refactor changed behaviour.

---

## Checklist

- [ ] `thiserror = { workspace = true }` added to the crate's `Cargo.toml`
- [ ] `{Service}Error` is now a `#[derive(Debug, Error)]` enum in `state.rs` with no HTTP status codes or AWS error type strings
- [ ] Every variant has an `#[error("...")]` attribute producing the original message text
- [ ] Every error construction in `state.rs` uses an enum variant
- [ ] The error-shaping function in `handlers.rs` has an exhaustive match (no wildcard arm)
- [ ] Every original `(error_type, message, status)` triple is preserved in the handler match
- [ ] No error helper methods remain in `state.rs` (replaced by enum variants)
- [ ] `./.agents/bin/cargo.sh build -p winterbaume-{service}` — no errors
- [ ] **Lint gate (mandatory before reporting)**: `./.agents/bin/cargo.sh clippy -p winterbaume-{service} --all-targets --all-features -- -D warnings` — passes. Fix any violations introduced by the refactor before finishing.
- [ ] **Fmt gate (mandatory before reporting)**: `./.agents/bin/cargo.sh fmt -p winterbaume-{service} -- --check` — passes.
- [ ] `./.agents/bin/cargo.sh test -p winterbaume-{service}` — all tests pass
- [ ] Wire-level behaviour is unchanged (same status codes, error codes, messages)
