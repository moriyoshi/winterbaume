#![recursion_limit = "256"]
//! Terraform state injection and extraction for winterbaume.
//!
//! This crate provides converters that map Terraform resource attributes
//! to winterbaume service state via the `StatefulService` trait. Use the
//! [`TerraformInjector`] to register converters and process a complete
//! Terraform state file.

pub mod converter;
pub mod converters;
pub mod error;
pub mod injector;
pub mod report;
pub mod util;

pub use converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
pub use error::ConversionError;
pub use injector::{ExtractionReport, TerraformInjector};
pub use report::InjectionReport;
