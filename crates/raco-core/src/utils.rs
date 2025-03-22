// Utility functions for RACO Core
//
// This module provides various utility functions used across the RACO system.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tracing::debug;

/// Ensures that a directory exists, creating it if necessary
pub fn ensure_dir_exists<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    let path_ref = path.as_ref();
    if !path_ref.exists() {
        debug!("Creating directory: {}", path_ref.display());
        std::fs::create_dir_all(path_ref)
            .with_context(|| format!("Failed to create directory: {}", path_ref.display()))?;
    } else if !path_ref.is_dir() {
        anyhow::bail!("Path exists but is not a directory: {}", path_ref.display());
    }

    Ok(path_ref.to_path_buf())
}

/// Returns a temporary directory for RACO
pub fn temp_dir() -> Result<PathBuf> {
    let temp_dir = std::env::temp_dir().join("raco");
    ensure_dir_exists(&temp_dir)?;
    Ok(temp_dir)
}

/// Checks if a file is valid JSON
pub fn is_valid_json<P: AsRef<Path>>(path: P) -> bool {
    if let Ok(content) = std::fs::read_to_string(path) {
        serde_json::from_str::<serde_json::Value>(&content).is_ok()
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    fn get_test_path(name: &str) -> PathBuf {
        let mut path = temp_dir().unwrap();
        path.push(format!("test_{}", name));
        path
    }

    #[test]
    fn test_ensure_dir_exists() {
        let test_dir = get_test_path("ensure_dir");
        // Remove if exists from previous test
        let _ = std::fs::remove_dir_all(&test_dir);

        let result = ensure_dir_exists(&test_dir);
        assert!(result.is_ok());
        assert!(test_dir.exists());
        assert!(test_dir.is_dir());

        // Clean up
        let _ = std::fs::remove_dir_all(&test_dir);
    }

    #[test]
    fn test_is_valid_json() {
        let test_file = get_test_path("valid.json");
        let mut file = File::create(&test_file).unwrap();
        writeln!(file, r#"{{"key": "value"}}"#).unwrap();

        assert!(is_valid_json(&test_file));

        let invalid_test_file = get_test_path("invalid.json");
        let mut file = File::create(&invalid_test_file).unwrap();
        writeln!(file, r#"{{"key": "value""#).unwrap();

        assert!(!is_valid_json(&invalid_test_file));

        // Clean up
        let _ = std::fs::remove_file(&test_file);
        let _ = std::fs::remove_file(&invalid_test_file);
    }
}
