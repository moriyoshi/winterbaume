//! Named, opt-in relaxations of AWS request validation.
//!
//! By default a service refuses what AWS refuses. A [`Relaxation`] names one
//! request that AWS rejects but that a service accepts when the relaxation is
//! enabled, usually because clients written against another AWS-compatible
//! server send it. Each relaxation is enabled on its own by name; there is no
//! bundle that turns several on at once.
//!
//! A service crate declares its relaxations as constants and lists them in a
//! `RELAXATIONS` slice. A service that has any takes the enabled set through a
//! `with_relaxations(&Relaxations)` builder method and picks out its own.
//!
//! ```
//! use winterbaume_core::{Relaxation, Relaxations};
//!
//! const EXAMPLE: Relaxation = Relaxation {
//!     name: "example-accept-something",
//!     summary: "Accept something AWS refuses.",
//! };
//! const KNOWN: &[Relaxation] = &[EXAMPLE];
//!
//! let enabled = Relaxations::from_names(["example-accept-something"], KNOWN).unwrap();
//! assert!(enabled.is_enabled(EXAMPLE));
//! assert!(Relaxations::from_names(["no-such-rule"], KNOWN).is_err());
//! ```

use std::collections::BTreeSet;

/// One named relaxation of a request check that AWS enforces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Relaxation {
    /// Stable name used on the command line and in configuration, in the form
    /// `<service>-<what is accepted>` ( e.g. `sts-assume-role-without-role-arn` ).
    pub name: &'static str,
    /// One-line description, shown in `--help` and logged when enabled.
    pub summary: &'static str,
}

/// A set of enabled relaxations. Empty by default.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Relaxations {
    enabled: BTreeSet<Relaxation>,
}

/// A relaxation name that no known relaxation carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownRelaxation {
    pub name: String,
}

impl std::fmt::Display for UnknownRelaxation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown relaxation '{}'", self.name)
    }
}

impl std::error::Error for UnknownRelaxation {}

impl Relaxations {
    /// An empty set: every service behaves as AWS does.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the set with `relaxation` enabled.
    pub fn with(mut self, relaxation: Relaxation) -> Self {
        self.enable(relaxation);
        self
    }

    /// Enables `relaxation`.
    pub fn enable(&mut self, relaxation: Relaxation) {
        self.enabled.insert(relaxation);
    }

    /// Whether `relaxation` is enabled.
    pub fn is_enabled(&self, relaxation: Relaxation) -> bool {
        self.enabled.contains(&relaxation)
    }

    /// Whether no relaxation is enabled.
    pub fn is_empty(&self) -> bool {
        self.enabled.is_empty()
    }

    /// The enabled relaxations, ordered by name.
    pub fn iter(&self) -> impl Iterator<Item = Relaxation> + '_ {
        self.enabled.iter().copied()
    }

    /// Resolves `names` against `known`. Surrounding whitespace is trimmed and
    /// empty names are skipped, so an empty environment variable enables
    /// nothing. Fails on the first name that `known` does not contain.
    pub fn from_names<'a>(
        names: impl IntoIterator<Item = &'a str>,
        known: &[Relaxation],
    ) -> Result<Self, UnknownRelaxation> {
        let mut set = Self::new();
        for name in names.into_iter().map(str::trim).filter(|n| !n.is_empty()) {
            let relaxation =
                known
                    .iter()
                    .find(|r| r.name == name)
                    .ok_or_else(|| UnknownRelaxation {
                        name: name.to_string(),
                    })?;
            set.enable(*relaxation);
        }
        Ok(set)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: Relaxation = Relaxation {
        name: "svc-a",
        summary: "A.",
    };
    const B: Relaxation = Relaxation {
        name: "svc-b",
        summary: "B.",
    };
    const KNOWN: &[Relaxation] = &[A, B];

    #[test]
    fn empty_by_default() {
        let set = Relaxations::default();
        assert!(set.is_empty());
        assert!(!set.is_enabled(A));
    }

    #[test]
    fn from_names_enables_only_the_named() {
        let set = Relaxations::from_names(["svc-b"], KNOWN).unwrap();
        assert!(set.is_enabled(B));
        assert!(!set.is_enabled(A));
    }

    #[test]
    fn from_names_trims_and_skips_empty() {
        let set = Relaxations::from_names(["", " svc-a ", "  "], KNOWN).unwrap();
        assert_eq!(set.iter().collect::<Vec<_>>(), vec![A]);
        assert!(Relaxations::from_names([""], KNOWN).unwrap().is_empty());
    }

    #[test]
    fn from_names_rejects_unknown() {
        let err = Relaxations::from_names(["svc-a", "svc-c"], KNOWN).unwrap_err();
        assert_eq!(err.name, "svc-c");
        assert_eq!(err.to_string(), "unknown relaxation 'svc-c'");
    }

    #[test]
    fn iter_is_ordered_by_name_without_duplicates() {
        let set = Relaxations::new().with(B).with(A).with(B);
        assert_eq!(set.iter().collect::<Vec<_>>(), vec![A, B]);
    }
}
