//! Shared protocol utilities for AWS service mock implementations.
//!
//! This module consolidates duplicated helper functions that were previously
//! copy-pasted across 20+ service crates. Three protocol families are supported:
//!
//! - **common**: URL parsing, percent-decoding, query string parsing
//! - **json**: awsJson1.0/1.1 error responses, field extraction helpers
//! - **xml**: XML escaping, awsQuery response envelope helpers

pub mod common;
pub mod json;
pub mod xml;

pub use common::{
    extract_path, extract_query_string, hex_val, parse_query_string, percent_decode, urldecode,
};
pub use json::{body_has_top_level_field, json_error_response, rest_json_error};
pub use xml::{aws_query_error_response, aws_query_response, xml_escape};
