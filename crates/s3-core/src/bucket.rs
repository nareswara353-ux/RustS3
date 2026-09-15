use crate::error::BucketError;
use crate::types::BucketName;
use chrono::{DateTime, Utc};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum VersioningStatus {
    Enabled,
    Suspended,
    Disabled,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bucket {
    pub name: BucketName,
    pub creation_date: DateTime<Utc>,
    pub location_constraint: Option<String>,
    pub versioning_status: VersioningStatus,
    pub tags: BTreeMap<String, String>,
    pub object_count: u64,
    pub total_size: u64,
}

impl Bucket {
    pub fn new(name_str: &str) -> Result<Self, BucketError> {
        let name = BucketName::try_from(name_str.to_string())?;
        
        if name_str.len() < 3 || name_str.len() > 63 {
            return Err(BucketError::InvalidNameLength(name_str.len()));
        }

        Ok(Self {
            name,
            creation_date: Utc::now(),
            location_constraint: None,
            versioning_status: VersioningStatus::Disabled,
            tags: BTreeMap::new(),
            object_count: 0,
            total_size: 0,
        })
    }

    pub fn with_location(mut self, constraint: String) -> Self {
        self.location_constraint = Some(constraint);
        self
    }

    pub fn enable_versioning(&mut self) {
        self.versioning_status = VersioningStatus::Enabled;
    }

    pub fn suspend_versioning(&mut self) {
        if self.versioning_status == VersioningStatus::Enabled {
            self.versioning_status = VersioningStatus::Suspended;
        }
    }

    pub fn disable_versioning(&mut self) {
        if self.object_count == 0 {
            self.versioning_status = VersioningStatus::Disabled;
        }
    }

    pub fn add_tag(&mut self, key: String, value: String) -> Result<(), BucketError> {
        if key.is_empty() || key.len() > 128 {
            return Err(BucketError::InvalidTagKey(key));
        }
        if value.len() > 256 {
            return Err(BucketError::InvalidTagValue(value));
        }
        self.tags.insert(key, value);
        Ok(())
    }

    pub fn remove_tag(&mut self, key: &str) {
        self.tags.remove(key);
    }

    pub fn increment_object_count(&mut self, size: u64) {
        self.object_count += 1;
        self.total_size += size;
    }

    pub fn decrement_object_count(&mut self, size: u64) {
        if self.object_count > 0 {
            self.object_count -= 1;
        }
        if self.total_size >= size {
            self.total_size -= size;
        } else {
            self.total_size = 0;
        }
    }

    pub fn is_versioned(&self) -> bool {
        self.versioning_status == VersioningStatus::Enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_valid_bucket() {
        let bucket = Bucket::new("my-test-bucket").unwrap();
        assert_eq!(bucket.name.as_str(), "my-test-bucket");
        assert_eq!(bucket.versioning_status, VersioningStatus::Disabled);
        assert!(bucket.tags.is_empty());
        assert_eq!(bucket.object_count, 0);
    }

    #[test]
    fn test_bucket_versioning_lifecycle() {
        let mut bucket = Bucket::new("versioned-bucket").unwrap();
        
        bucket.enable_versioning();
        assert_eq!(bucket.versioning_status, VersioningStatus::Enabled);
        
        bucket.suspend_versioning();
        assert_eq!(bucket.versioning_status, VersioningStatus::Suspended);
        
        bucket.disable_versioning();
        assert_eq!(bucket.versioning_status, VersioningStatus::Suspended);
        
        bucket.enable_versioning();
        bucket.disable_versioning();
        assert_eq!(bucket.versioning_status, VersioningStatus::Suspended);
    }

    #[test]
    fn test_bucket_tags() {
        let mut bucket = Bucket::new("tagged-bucket").unwrap();
        
        bucket.add_tag("Environment".to_string(), "Production".to_string()).unwrap();
        bucket.add_tag("Team".to_string(), "Backend".to_string()).unwrap();
        
        assert_eq!(bucket.tags.len(), 2);
        assert_eq!(bucket.tags.get("Environment"), Some(&"Production".to_string()));
        
        bucket.remove_tag("Team");
        assert_eq!(bucket.tags.len(), 1);
        
        let result = bucket.add_tag("".to_string(), "val".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_bucket_metrics() {
        let mut bucket = Bucket::new("metrics-bucket").unwrap();
        
        bucket.increment_object_count(1024);
        bucket.increment_object_count(2048);
        
        assert_eq!(bucket.object_count, 2);
        assert_eq!(bucket.total_size, 3072);
        
        bucket.decrement_object_count(1024);
        assert_eq!(bucket.object_count, 1);
        assert_eq!(bucket.total_size, 2048);
    }
}
