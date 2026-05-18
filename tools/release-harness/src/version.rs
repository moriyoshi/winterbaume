//! Minimal semver representation just rich enough for the harness.
//!
//! We deliberately do not depend on the `semver` crate: per-crate bumps under
//! the project's 0.y.z convention only need parse / pretty-print / bump-by-level,
//! and avoiding the dep keeps the harness binary small.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl FromStr for Version {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(3, '.');
        let major = parts
            .next()
            .ok_or_else(|| format!("missing major in {s:?}"))?
            .parse::<u32>()
            .map_err(|e| format!("major in {s:?}: {e}"))?;
        let minor = parts
            .next()
            .ok_or_else(|| format!("missing minor in {s:?}"))?
            .parse::<u32>()
            .map_err(|e| format!("minor in {s:?}: {e}"))?;
        let patch_raw = parts
            .next()
            .ok_or_else(|| format!("missing patch in {s:?}"))?;
        let cut = patch_raw.find(['-', '+']).unwrap_or(patch_raw.len());
        let patch = patch_raw[..cut]
            .parse::<u32>()
            .map_err(|e| format!("patch in {s:?}: {e}"))?;
        Ok(Self {
            major,
            minor,
            patch,
        })
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Bump levels recognised by the harness, sorted by ascending impact.
///
/// Note: under the project's 0.y.z convention, a `Major` resolves to a
/// `0.y.0 → 0.(y+1).0` step (matching how the v0.1.0 → v0.2.0 batch shipped).
/// `Minor` resolves to `0.y.z → 0.(y+1).0` as well when `major == 0` for
/// strict semver compatibility (pre-1.0 minor *is* the breaking-change axis),
/// while `Patch` resolves to `0.y.z → 0.y.(z+1)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Patch,
    Minor,
    Major,
}

impl Level {
    pub fn cargo_release_keyword(self) -> &'static str {
        match self {
            Level::Patch => "patch",
            Level::Minor => "minor",
            Level::Major => "major",
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.cargo_release_keyword())
    }
}

impl Version {
    /// Apply a semver level bump using the same rules cargo-release applies.
    /// Under 0.y.z, `major` and `minor` both bump the minor axis (per semver
    /// pre-1.0 convention); `patch` always bumps the patch axis.
    pub fn bumped(self, level: Level) -> Self {
        match (self.major, level) {
            (0, Level::Major) | (0, Level::Minor) => Self {
                major: 0,
                minor: self.minor + 1,
                patch: 0,
            },
            (_, Level::Major) => Self {
                major: self.major + 1,
                minor: 0,
                patch: 0,
            },
            (_, Level::Minor) => Self {
                major: self.major,
                minor: self.minor + 1,
                patch: 0,
            },
            (_, Level::Patch) => Self {
                major: self.major,
                minor: self.minor,
                patch: self.patch + 1,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_renders() {
        let v: Version = "0.2.0".parse().unwrap();
        assert_eq!(v.to_string(), "0.2.0");
        let v: Version = "1.4.7".parse().unwrap();
        assert_eq!((v.major, v.minor, v.patch), (1, 4, 7));
    }

    #[test]
    fn parses_with_prerelease() {
        let v: Version = "0.3.0-rc1".parse().unwrap();
        assert_eq!((v.major, v.minor, v.patch), (0, 3, 0));
    }

    #[test]
    fn pre_1_0_minor_and_major_both_bump_minor_axis() {
        let v: Version = "0.2.0".parse().unwrap();
        assert_eq!(v.bumped(Level::Minor).to_string(), "0.3.0");
        assert_eq!(v.bumped(Level::Major).to_string(), "0.3.0");
        assert_eq!(v.bumped(Level::Patch).to_string(), "0.2.1");
    }

    #[test]
    fn post_1_0_levels_are_distinct() {
        let v: Version = "1.4.7".parse().unwrap();
        assert_eq!(v.bumped(Level::Patch).to_string(), "1.4.8");
        assert_eq!(v.bumped(Level::Minor).to_string(), "1.5.0");
        assert_eq!(v.bumped(Level::Major).to_string(), "2.0.0");
    }
}
