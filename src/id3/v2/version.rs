#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Version {
    V2,
    V3,
    V4,
}

impl From<u8> for Version {
    fn from(value: u8) -> Self {
        match value {
            2 => Version::V2,
            4 => Version::V4,
            _ => Version::V3,
        }
    }
}

impl From<Version> for u8 {
    fn from(version: Version) -> Self {
        match version {
            Version::V2 => 2,
            Version::V3 => 3,
            Version::V4 => 4,
        }
    }
}
