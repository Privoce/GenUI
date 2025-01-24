use std::{fmt::Display, str::FromStr};

use crate::error::{Error, ParseError};

/// # Standard Version
/// ## format
/// `major.minor.patch` such as `1.2.3`
/// ## parse (from_str)
/// - if the version is `1.2.3` it will parse to `Version { major: 1, minor: 2, patch: 3 }`
/// - if the version is `1.2` it will parse to `Version { major: 1, minor: 2, patch: 0 }`
/// - if the version is `1` it will parse to `Version { major: 1, minor: 0, patch: 0 }`
/// - if the version is `0.1` it will parse to `Version { major: 0, minor: 1, patch: 0 }`
///
/// **that means you can also use this struct to handle the version string which is not standard.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    /// major version, if major version is 0, it means the version is not stable
    pub major: u32,
    /// minor version
    pub minor: u32,
    /// patch version
    pub patch: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
    /// Compare the major version of the current version with the given version.
    /// - Returns 0 if the major version is the same
    /// - a positive number if the current version is greater
    /// - a negative number if the current version is less.
    pub fn match_major(&self, major: u32) -> i32 {
        self.major as i32 - major as i32
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.major, self.minor, self.patch).cmp(&(other.major, other.minor, other.patch))
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_version_item(item: Option<Result<u32, Error>>) -> Result<u32, Error> {
            match item {
                Some(item) => item,
                None => Ok(0),
            }
        }

        let mut iter = s.split('.').map(|x| {
            x.parse::<u32>().map_err(|e| {
                let mut err = ParseError::rust_dep(x);
                err.set_other(&format!("can not parse {} to version item", e));
                err.into()
            })
        });

        let major = parse_version_item(iter.next())?;
        let minor = parse_version_item(iter.next())?;
        let patch = parse_version_item(iter.next())?;

        Ok(Version::new(major, minor, patch))
    }
}

#[cfg(test)]
mod test_version {
    #[test]
    fn from() {
        let v = "1.2.3".parse::<crate::compiler::Version>().unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
    }
    #[test]
    fn from2() {
        let v = "1.2".parse::<crate::compiler::Version>().unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 0);
    }
}
