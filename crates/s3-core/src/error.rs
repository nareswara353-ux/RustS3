use thiserror::Error;

#[derive(Error, Debug)]
pub enum S3CoreError {
    #[error("Invalid XML structure: {message}")]
    InvalidXml { message: String },

    #[error("Missing required field: {field_name}")]
    MissingField { field_name: String },

    #[error("Invalid bucket name: {name} - {reason}")]
    InvalidBucketName { name: String, reason: String },

    #[error("Invalid object key: {key} - {reason}")]
    InvalidObjectKey { key: String, reason: String },

    #[error("Metadata validation failed: {message}")]
    MetadataError { message: String },

    #[error("Encoding error: {message}")]
    EncodingError { message: String },

    #[error("Timestamp parse failure: {source}")]
    TimestampError {
        #[from]
        source: chrono::ParseError,
    },

    #[error("UUID generation failed: {source}")]
    UuidError {
        #[from]
        source: uuid::Error,
    },
}

pub type Result<T> = std::result::Result<T, S3CoreError>;
