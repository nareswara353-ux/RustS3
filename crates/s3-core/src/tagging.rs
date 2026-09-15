use thiserror::Error;
use std::collections::BTreeMap;

#[derive(Error, Debug)]
pub enum TaggingError {
    #[error("Tag set exceeds maximum limit of 10 tags")]
    TooManyTags,
    #[error("Tag key '{0}' exceeds 128 bytes")]
    KeyTooLong(String),
    #[error("Tag value '{0}' exceeds 256 bytes")]
    ValueTooLong(String),
    #[error("Tag key contains invalid characters")]
    InvalidKeyFormat,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

impl Tag {
    pub fn new(key: String, value: String) -> Result<Self, TaggingError> {
        if key.is_empty() {
            return Err(TaggingError::InvalidKeyFormat);
        }
        if key.len() > 128 {
            return Err(TaggingError::KeyTooLong(key));
        }
        if value.len() > 256 {
            return Err(TaggingError::ValueTooLong(value));
        }
        
        let valid_chars = key.chars().all(|c| {
            c.is_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '/' || c == ':' || c == '+' || c == '=' || c == '@' || c == ' '
        });
        
        if !valid_chars {
            return Err(TaggingError::InvalidKeyFormat);
        }

        Ok(Self { key, value })
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TagSet {
    tags: BTreeMap<String, String>,
}

impl TagSet {
    pub fn new() -> Self {
        Self { tags: BTreeMap::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            tags: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: String) -> Result<(), TaggingError> {
        if self.tags.len() >= 10 {
            return Err(TaggingError::TooManyTags);
        }
        
        let tag = Tag::new(key.clone(), value.clone())?;
        self.tags.insert(tag.key, tag.value);
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.tags.get(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.tags.remove(key)
    }

    pub fn len(&self) -> usize {
        self.tags.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.tags.iter()
    }

    pub fn validate(&self) -> Result<(), TaggingError> {
        if self.tags.len() > 10 {
            return Err(TaggingError::TooManyTags);
        }
        for (key, value) in &self.tags {
            if key.len() > 128 {
                return Err(TaggingError::KeyTooLong(key.clone()));
            }
            if value.len() > 256 {
                return Err(TaggingError::ValueTooLong(value.clone()));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_tag_creation() {
        let tag = Tag::new("Environment".to_string(), "Production".to_string()).unwrap();
        assert_eq!(tag.key, "Environment");
        assert_eq!(tag.value, "Production");
    }

    #[test]
    fn test_tag_key_too_long() {
        let long_key = "k".repeat(129);
        assert!(Tag::new(long_key, "val".to_string()).is_err());
    }

    #[test]
    fn test_tag_value_too_long() {
        let long_val = "v".repeat(257);
        assert!(Tag::new("key".to_string(), long_val).is_err());
    }

    #[test]
    fn test_tag_set_limit() {
        let mut tag_set = TagSet::new();
        for i in 0..10 {
            tag_set.insert(format!("key{}", i), format!("val{}", i)).unwrap();
        }
        assert!(tag_set.insert("extra".to_string(), "value".to_string()).is_err());
    }

    #[test]
    fn test_tag_set_validation() {
        let mut tag_set = TagSet::new();
        tag_set.insert("Project".to_string(), "S3Mock".to_string()).unwrap();
        assert!(tag_set.validate().is_ok());
        assert_eq!(tag_set.len(), 1);
        assert_eq!(tag_set.get("Project"), Some(&"S3Mock".to_string()));
    }
}
