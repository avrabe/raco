//! RACO MCP - Model Context Protocol implementation
//!
//! This crate provides an implementation of the Model Context Protocol (MCP) for RACO.
//! It includes server management, client interfaces, and protocol handlers.

pub mod client;
pub mod protocol;
pub mod server;

/// Current version of the RACO MCP library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns the version of the RACO MCP library
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
