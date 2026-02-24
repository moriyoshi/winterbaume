//! Injection report types.

use crate::error::ConversionError;

/// Summary of a Terraform state injection operation.
#[derive(Debug, Default)]
pub struct InjectionReport {
    /// Number of resource instances successfully injected.
    pub injected: usize,
    /// Resource types that were skipped (no converter registered).
    pub skipped: Vec<String>,
    /// Non-fatal warnings from converters.
    pub warnings: Vec<String>,
    /// Errors encountered during conversion.
    pub errors: Vec<ConversionError>,
}

impl InjectionReport {
    /// Returns `true` if any errors were recorded.
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Returns `true` if no errors were recorded.
    pub fn is_success(&self) -> bool {
        self.errors.is_empty()
    }
}

impl std::fmt::Display for InjectionReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Injected: {}", self.injected)?;
        if !self.skipped.is_empty() {
            write!(f, ", Skipped types: {}", self.skipped.join(", "))?;
        }
        if !self.warnings.is_empty() {
            write!(f, ", Warnings: {}", self.warnings.len())?;
            for w in &self.warnings {
                write!(f, "\n  WARN: {w}")?;
            }
        }
        if !self.errors.is_empty() {
            write!(f, ", Errors: {}", self.errors.len())?;
            for e in &self.errors {
                write!(f, "\n  ERROR: {e}")?;
            }
        }
        Ok(())
    }
}
