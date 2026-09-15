use crate::error::CoreError;
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BucketName(String);

impl BucketName {
    pub fn new(name: &str) -> Result<Self, CoreError> {
        if name.is_empty() || name.len() > 63 {
            return Err(CoreError::InvalidBucketName("length must be 1-63".into()));
        }
        if !name.starts_with(|c: char| c.is_ascii_lowercase() || c.is_ascii_digit()) {
            return Err(CoreError::InvalidBucketName("must start with lowercase letter or digit".into()));
        }
        if !name.ends_with(|c: char| c.is_ascii_lowercase() || c.is_ascii_digit()) {
            return Err(CoreError::InvalidBucketName("must end with lowercase letter or digit".into()));
        }
        if name.contains("..") || name.contains(".-") || name.contains("-.") {
            return Err(CoreError::InvalidBucketName("cannot have consecutive dots or hyphens with dots".into()));
        }
        if !name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '.' || c == '-') {
            return Err(CoreError::InvalidBucketName("only lowercase letters, digits, dots, and hyphens allowed".into()));
        }
        Ok(Self(name.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for BucketName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObjectName(String);

impl ObjectName {
    pub fn new(name: &str) -> Result<Self, CoreError> {
        if name.is_empty() || name.len() > 1024 {
            return Err(CoreError::InvalidObjectName("length must be 1-1024".into()));
        }
        Ok(Self(name.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ObjectName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ETag(String);

impl ETag {
    pub fn new(etag: &str) -> Result<Self, CoreError> {
        let clean = etag.trim_matches('"');
        if clean.is_empty() || clean.len() > 64 {
            return Err(CoreError::InvalidMetadata("invalid etag format".into()));
        }
        Ok(Self(format!("\"{}\"", clean)))
    }

    pub fn from_hash(hash: &str) -> Self {
        Self(format!("\"{}\"", hash))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ETag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VersionId(String);

impl VersionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn from_string(id: &str) -> Self {
        Self(id.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for VersionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RequestId(Uuid);

impl RequestId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl fmt::Display for RequestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for RequestId {
    fn default() -> Self {
        Self::new()
    }
}
