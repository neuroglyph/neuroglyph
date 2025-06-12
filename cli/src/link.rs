// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! Link representation and operations

use sha1::{Digest, Sha1};
use std::fmt;

/// Represents a semantic link between two files
#[derive(Debug, PartialEq)]
pub struct Link {
    pub link_type: String,
    pub source: String,
    pub target: String,
    pub timestamp: i64,
}

impl Link {
    /// Create a new link
    pub fn new(link_type: String, source: String, target: String, timestamp: i64) -> Self {
        Self {
            link_type,
            source,
            target,
            timestamp,
        }
    }

    /// Convert to canonical string format
    pub fn to_canonical_string(&self) -> String {
        format!(
            "{}: {} -> {}  # ts:{}",
            self.link_type, self.source, self.target, self.timestamp
        )
    }

    /// Calculate SHA-1 hash of the canonical representation
    pub fn sha(&self) -> String {
        let mut hasher = Sha1::new();
        hasher.update(self.to_canonical_string().as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Get the short SHA (first 8 characters)
    pub fn short_sha(&self) -> String {
        self.sha()[..8].to_string()
    }

    /// Parse a link from its canonical string representation
    pub fn from_canonical_string(s: &str) -> crate::error::Result<Self> {
        // Expected format: "LINK_TYPE: source -> target  # ts:timestamp"
        let parts: Vec<&str> = s.split(" -> ").collect();
        if parts.len() != 2 {
            return Err(crate::error::Error::ParseError(
                "Invalid link format".to_string(),
            ));
        }

        let type_and_source: Vec<&str> = parts[0].split(": ").collect();
        if type_and_source.len() != 2 {
            return Err(crate::error::Error::ParseError(
                "Invalid link type/source format".to_string(),
            ));
        }

        let target_and_ts: Vec<&str> = parts[1].split("  # ts:").collect();
        if target_and_ts.len() != 2 {
            return Err(crate::error::Error::ParseError(
                "Invalid target/timestamp format".to_string(),
            ));
        }

        let timestamp = target_and_ts[1]
            .parse::<i64>()
            .map_err(|_| crate::error::Error::ParseError("Invalid timestamp".to_string()))?;

        Ok(Link::new(
            type_and_source[0].to_string(),
            type_and_source[1].to_string(),
            target_and_ts[0].to_string(),
            timestamp,
        ))
    }
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_canonical_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_canonical_string() {
        let link = Link::new(
            "CROSS_REF".to_string(),
            "doc/api.md".to_string(),
            "src/lib.rs".to_string(),
            1736637876,
        );
        assert_eq!(
            link.to_canonical_string(),
            "CROSS_REF: doc/api.md -> src/lib.rs  # ts:1736637876"
        );
    }

    #[test]
    fn test_link_sha_is_deterministic() {
        let link1 = Link::new(
            "CROSS_REF".to_string(),
            "a.md".to_string(),
            "b.md".to_string(),
            1234567890,
        );
        let link2 = Link::new(
            "CROSS_REF".to_string(),
            "a.md".to_string(),
            "b.md".to_string(),
            1234567890,
        );
        assert_eq!(link1.sha(), link2.sha());
    }

    #[test]
    fn test_link_from_canonical_string() {
        let canonical = "IMPLEMENTS: spec.md -> code.rs  # ts:1736637876";
        let link = Link::from_canonical_string(canonical).unwrap();
        assert_eq!(link.link_type, "IMPLEMENTS");
        assert_eq!(link.source, "spec.md");
        assert_eq!(link.target, "code.rs");
        assert_eq!(link.timestamp, 1736637876);
    }
}
