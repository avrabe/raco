// RACO Core - Central functionality and shared components
//
// This crate provides the core functionality and shared components for RACO.
// It includes configuration management, error handling, and common utilities.

pub mod config;
pub mod error;
pub mod utils;

/// Current version of the RACO Core library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns the version of the RACO Core library
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
