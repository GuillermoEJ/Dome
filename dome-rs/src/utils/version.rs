/// Version comparison and parsing
/// 
/// Semantic versioning utilities

use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    /// Parse version from string (e.g., "1.2.3")
    pub fn parse(version_str: &str) -> Option<Self> {
        let parts: Vec<&str> = version_str.split('.').collect();
        if parts.len() != 3 {
            return None;
        }

        Some(Version {
            major: parts[0].parse().ok()?,
            minor: parts[1].parse().ok()?,
            patch: parts[2].parse().ok()?,
        })
    }

    /// Format version to string
    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.major
            .cmp(&other.major)
            .then(self.minor.cmp(&other.minor))
            .then(self.patch.cmp(&other.patch))
    }
}

/// Compare two version strings
/// Returns: -1 if v1 < v2, 0 if equal, 1 if v1 > v2
pub fn compare_versions(v1: &str, v2: &str) -> i32 {
    match (Version::parse(v1), Version::parse(v2)) {
        (Some(ver1), Some(ver2)) => match ver1.cmp(&ver2) {
            Ordering::Less => -1,
            Ordering::Greater => 1,
            Ordering::Equal => 0,
        },
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_parse() {
        let v = Version::parse("1.2.3").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
    }

    #[test]
    fn test_version_comparison() {
        assert_eq!(compare_versions("1.0.0", "2.0.0"), -1);
        assert_eq!(compare_versions("2.0.0", "1.0.0"), 1);
        assert_eq!(compare_versions("1.5.0", "1.5.0"), 0);
    }
}
