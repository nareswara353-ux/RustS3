use crate::types::{BucketName, ObjectName};
use chrono::Duration;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LifecycleError {
    #[error("Invalid transition days: {0}")]
    InvalidTransitionDays(u32),
    #[error("Invalid expiration days: {0}")]
    InvalidExpirationDays(u32),
    #[error("Duplicate storage class transition")]
    DuplicateTransition,
    #[error("Invalid filter configuration")]
    InvalidFilter,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum StorageClass {
    Standard,
    StandardIa,
    OneZoneIa,
    IntelligentTiering,
    Glacier,
    GlacierIr,
    DeepArchive,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Transition {
    pub days: Option<u32>,
    pub date: Option<chrono::DateTime<chrono::Utc>>,
    pub storage_class: StorageClass,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Expiration {
    pub days: Option<u32>,
    pub date: Option<chrono::DateTime<chrono::Utc>>,
    pub expired_object_delete_marker: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LifecycleFilter {
    pub prefix: Option<String>,
    pub tag: Option<Tag>,
    pub and: Option<AndOperator>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Tag {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct AndOperator {
    pub prefix: Option<String>,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LifecycleRule {
    pub id: Option<String>,
    pub status: RuleStatus,
    pub priority: u32,
    pub filter: LifecycleFilter,
    pub transition: Option<Transition>,
    pub expiration: Option<Expiration>,
    pub noncurrent_version_transition: Option<Transition>,
    pub noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum RuleStatus {
    Enabled,
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct NoncurrentVersionExpiration {
    pub noncurrent_days: u32,
}

impl Transition {
    pub fn new(days: u32, storage_class: StorageClass) -> Result<Self, LifecycleError> {
        if days == 0 {
            return Err(LifecycleError::InvalidTransitionDays(days));
        }
        Ok(Self {
            days: Some(days),
            date: None,
            storage_class,
        })
    }
}

impl Expiration {
    pub fn new(days: u32) -> Result<Self, LifecycleError> {
        if days == 0 {
            return Err(LifecycleError::InvalidExpirationDays(days));
        }
        Ok(Self {
            days: Some(days),
            date: None,
            expired_object_delete_marker: None,
        })
    }
}

impl LifecycleRule {
    pub fn validate(&self) -> Result<(), LifecycleError> {
        if let Some(transition) = &self.transition {
            if let Some(days) = transition.days {
                if days == 0 {
                    return Err(LifecycleError::InvalidTransitionDays(days));
                }
            }
        }
        if let Some(expiration) = &self.expiration {
            if let Some(days) = expiration.days {
                if days == 0 {
                    return Err(LifecycleError::InvalidExpirationDays(days));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transition() {
        let t = Transition::new(30, StorageClass::Glacier).unwrap();
        assert_eq!(t.days, Some(30));
        assert_eq!(t.storage_class, StorageClass::Glacier);
    }

    #[test]
    fn test_invalid_zero_days_transition() {
        assert!(Transition::new(0, StorageClass::StandardIa).is_err());
    }

    #[test]
    fn test_lifecycle_rule_validation() {
        let rule = LifecycleRule {
            id: Some("rule1".to_string()),
            status: RuleStatus::Enabled,
            priority: 1,
            filter: LifecycleFilter {
                prefix: Some("logs/".to_string()),
                tag: None,
                and: None,
            },
            transition: Some(Transition::new(90, StorageClass::Glacier).unwrap()),
            expiration: Some(Expiration::new(365).unwrap()),
            noncurrent_version_transition: None,
            noncurrent_version_expiration: None,
        };
        assert!(rule.validate().is_ok());
    }
}
