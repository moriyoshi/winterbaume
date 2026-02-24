//! Internal domain types for DynamoDB Streams.
//!
//! The service currently keeps its concrete stream state in `state.rs` because
//! streams are derived from DynamoDB table state. This module exists to preserve
//! the canonical service-crate layout as durable types are added.
