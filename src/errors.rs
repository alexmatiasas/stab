use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum StabError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to read file '{path}': {source}")]
    FileRead {
        path: PathBuf,
        #[source]
        source: polars::error::PolarsError,
    },

    #[error("Unsupported file format: '{0}'")]
    UnsupportedFormat(String),

    #[error("Column '{0}' not found in DataFrame")]
    ColumnNotFound(String),

    #[error("Display error: {0}")]
    Display(String),
}

// Convenience alias — instead of writing Result<T, StabError> in all locations
pub type StabResult<T> = std::result::Result<T, StabError>;
