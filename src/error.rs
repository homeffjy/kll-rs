//! Error types for DataSketches operations.

use std::fmt;

/// Error type for DataSketches operations.
#[derive(Debug, Clone)]
pub enum DataSketchesError {
    /// An error occurred during sketch creation.
    CreationError(String),
    /// An error occurred during serialization.
    SerializationError(String),
    /// An error occurred during deserialization.
    DeserializationError(String),
    /// An invalid parameter was provided.
    InvalidParameter(String),
    /// A null pointer was encountered.
    NullPointer,
    /// An unknown error occurred.
    Unknown(String),
}

impl fmt::Display for DataSketchesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataSketchesError::CreationError(msg) => write!(f, "Sketch creation error: {}", msg),
            DataSketchesError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            DataSketchesError::DeserializationError(msg) => {
                write!(f, "Deserialization error: {}", msg)
            }
            DataSketchesError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
            DataSketchesError::NullPointer => write!(f, "Null pointer encountered"),
            DataSketchesError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for DataSketchesError {}

pub type Result<T> = std::result::Result<T, DataSketchesError>;
