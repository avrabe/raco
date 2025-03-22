// Error handling for RACO Core
//
// This module provides error types and utility functions for error handling.

use thiserror::Error;

/// Core error types
#[derive(Error, Debug)]
pub enum CoreError {
    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// I/O errors
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization errors
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// General error with a message
    #[error("{0}")]
    Other(String),
}

/// Result type alias for CoreError
pub type CoreResult<T> = Result<T, CoreError>;

/// Utility function to convert an error to a core error with a message
pub fn with_context<T, E: std::fmt::Display>(
    result: Result<T, E>,
    context: impl FnOnce() -> String,
) -> CoreResult<T> {
    result.map_err(|e| CoreError::Other(format!("{}: {}", context(), e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_io_error_conversion() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let core_err: CoreError = io_err.into();

        match core_err {
            CoreError::Io(_) => (),
            _ => panic!("Expected Io error variant"),
        }
    }

    #[test]
    fn test_with_context() {
        let result: Result<(), io::Error> = Err(io::Error::new(io::ErrorKind::Other, "test error"));

        let core_result = with_context(result, || "Context message".to_string());

        assert!(core_result.is_err());
        match core_result {
            Err(CoreError::Other(msg)) => {
                assert!(msg.contains("Context message"));
                assert!(msg.contains("test error"));
            }
            _ => panic!("Expected Other error variant with context message"),
        }
    }
}
