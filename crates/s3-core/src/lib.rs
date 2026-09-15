#![deny(clippy::all)]
#![warn(missing_docs)]

pub mod error;
pub mod types;
pub mod xml;

pub use error::{S3Error, Result};
pub use types::{Bucket, Object, ObjectKey, ETag, VersionId};
