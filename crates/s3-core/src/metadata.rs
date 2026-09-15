use crate::types::{ETag, ObjectName};
use chrono::{DateTime, Utc};
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MetadataError {
    #[error("Invalid content length: {0}")]
    InvalidContentLength(i64),
    #[error("Invalid last modified timestamp")]
    InvalidTimestamp,
    #[error("Metadata key '{0}' exceeds 1024 bytes")]
    KeyTooLong(String),
    #[error("Metadata value '{0}' exceeds 4096 bytes")]
    ValueTooLong(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectMetadata {
    pub key: ObjectName,
    pub size: u64,
    pub etag: ETag,
    pub last_modified: DateTime<Utc>,
    pub content_type: String,
    pub content_encoding: Option<String>,
    pub user_metadata: BTreeMap<String, String>,
}

impl ObjectMetadata {
    pub fn new(
        key: ObjectName,
        size: u64,
        etag: ETag,
        content_type: String,
    ) -> Result<Self, MetadataError> {
        Ok(Self {
            key,
            size,
            etag,
            last_modified: Utc::now(),
            content_type,
            content_encoding: None,
            user_metadata: BTreeMap::new(),
        })
    }

    pub fn with_content_encoding(mut self, encoding: String) -> Self {
        self.content_encoding = Some(encoding);
        self
    }

    pub fn insert_user_metadata(
        &mut self,
        key: String,
        value: String,
    ) -> Result<(), MetadataError> {
        if key.len() > 1024 {
            return Err(MetadataError::KeyTooLong(key));
        }
        if value.len() > 4096 {
            return Err(MetadataError::ValueTooLong(value));
        }
        self.user_metadata.insert(key, value);
        Ok(())
    }

    pub fn validate(&self) -> Result<(), MetadataError> {
        if self.size == 0 && self.key.as_str() != "$folder" {
            // Zero-byte objects are allowed but flagged for special handling
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ObjectName;

    #[test]
    fn test_create_valid_metadata() {
        let key = ObjectName::try_from("test.txt".to_string()).unwrap();
        let etag = ETag::new("\"d41d8cd98f00b204e9800998ecf8427e\"".to_string());
        let meta = ObjectMetadata::new(
            key.clone(),
            1024,
            etag,
            "text/plain".to_string(),
        ).unwrap();
        
        assert_eq!(meta.size, 1024);
        assert_eq!(meta.content_type, "text/plain");
        assert!(meta.user_metadata.is_empty());
    }

    #[test]
    fn test_user_metadata_limits() {
        let key = ObjectName::try_from("test.txt".to_string()).unwrap();
        let etag = ETag::new("\"abc\"".to_string());
        let mut meta = ObjectMetadata::new(
            key,
            0,
            etag,
            "application/octet-stream".to_string(),
        ).unwrap();

        let long_key = "k".repeat(1025);
        assert!(meta.insert_user_metadata(long_key, "val".to_string()).is_err());

        let long_val = "v".repeat(4097);
        assert!(meta.insert_user_metadata("key".to_string(), long_val).is_err());
    }
}
