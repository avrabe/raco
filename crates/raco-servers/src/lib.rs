//! RACO Servers - MCP server implementations
//!
//! This crate provides implementations of various MCP servers for RACO.

pub mod filesystem;
pub mod process;
pub mod registry;

use raco_core::error::CoreError;
use thiserror::Error;

/// Error type for server operations
#[derive(Error, Debug)]
pub enum ServerError {
    /// Core error
    #[error("Core error: {0}")]
    Core(#[from] CoreError),

    /// MCP error
    #[error("MCP error: {0}")]
    Mcp(String),

    /// Server not found
    #[error("Server not found: {0}")]
    ServerNotFound(String),

    /// Operation not supported
    #[error("Operation not supported: {0}")]
    NotSupported(String),

    /// General error
    #[error("{0}")]
    General(String),
}

/// Result type for server operations
pub type ServerResult<T> = Result<T, ServerError>;

/// Current version of the RACO Servers library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns the version of the RACO Servers library
pub fn version() -> &'static str {
    VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(version(), VERSION);
    }
}
