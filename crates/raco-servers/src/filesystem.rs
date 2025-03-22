//! Filesystem MCP server
//!
//! This module provides an MCP server implementation for filesystem operations.

use raco_mcp::protocol::{FileInfo, McpRequest, McpResponse, ResponseStatus};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tracing::{debug, error, info};

use crate::ServerResult;

/// Filesystem server for handling filesystem operations
#[derive(Debug)]
pub struct FilesystemServer {
    /// Root directory for filesystem operations
    #[allow(dead_code)]
    root_dir: PathBuf,

    /// Server ID
    id: String,
}

/// Filesystem command types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FilesystemCommand {
    /// List files in a directory
    #[serde(rename = "list")]
    List {
        /// Path to list
        path: String,

        /// Whether to recurse into subdirectories
        #[serde(default)]
        recursive: bool,
    },

    /// Read a file
    #[serde(rename = "read")]
    Read {
        /// Path to read
        path: String,

        /// Optional encoding (defaults to UTF-8)
        #[serde(default)]
        encoding: Option<String>,
    },

    /// Write to a file
    #[serde(rename = "write")]
    Write {
        /// Path to write
        path: String,

        /// Content to write
        content: String,

        /// Whether to append to the file (defaults to false)
        #[serde(default)]
        append: bool,
    },

    /// Delete a file or directory
    #[serde(rename = "delete")]
    Delete {
        /// Path to delete
        path: String,

        /// Whether to recursively delete directories
        #[serde(default)]
        recursive: bool,
    },
}

/// Filesystem response types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FilesystemResponse {
    /// List response
    #[serde(rename = "list")]
    List {
        /// Files in the directory
        files: Vec<FileInfo>,
    },

    /// Read response
    #[serde(rename = "read")]
    Read {
        /// Content of the file
        content: String,

        /// Encoding used
        encoding: String,
    },

    /// Write response
    #[serde(rename = "write")]
    Write {
        /// Bytes written
        bytes_written: u64,
    },

    /// Delete response
    #[serde(rename = "delete")]
    Delete {
        /// Whether the deletion was successful
        success: bool,
    },
}

impl FilesystemServer {
    /// Create a new filesystem server with the given root directory
    pub fn new<P: AsRef<Path>>(root_dir: P) -> Self {
        let root_dir = root_dir.as_ref().to_path_buf();
        info!(
            "Creating filesystem server with root directory: {}",
            root_dir.display()
        );
        Self {
            root_dir,
            id: uuid::Uuid::new_v4().to_string(),
        }
    }

    /// Get the server ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Handle an MCP request
    pub async fn handle_request(
        &self,
        request: McpRequest<FilesystemCommand>,
    ) -> ServerResult<McpResponse<FilesystemResponse>> {
        debug!("Handling filesystem request: {:?}", request);

        let response = match request.payload {
            FilesystemCommand::List { path, recursive } => self.handle_list(path, recursive).await,
            FilesystemCommand::Read { path, encoding } => self.handle_read(path, encoding).await,
            FilesystemCommand::Write {
                path,
                content,
                append,
            } => self.handle_write(path, content, append).await,
            FilesystemCommand::Delete { path, recursive } => {
                self.handle_delete(path, recursive).await
            }
        };

        let response = match response {
            Ok(payload) => McpResponse {
                command: request.command,
                payload,
                status: ResponseStatus::success(),
                request_id: request.request_id,
            },
            Err(e) => {
                error!("Error handling filesystem request: {}", e);
                McpResponse {
                    command: request.command,
                    payload: create_error_response(&e.to_string()),
                    status: ResponseStatus::error(1, &e.to_string()),
                    request_id: request.request_id,
                }
            }
        };

        Ok(response)
    }

    // Implementation of the handlers will go here in a real implementation
    async fn handle_list(
        &self,
        _path: String,
        _recursive: bool,
    ) -> Result<FilesystemResponse, anyhow::Error> {
        // This is a placeholder - actual implementation would list files
        Ok(FilesystemResponse::List { files: vec![] })
    }

    async fn handle_read(
        &self,
        _path: String,
        _encoding: Option<String>,
    ) -> Result<FilesystemResponse, anyhow::Error> {
        // This is a placeholder - actual implementation would read the file
        Ok(FilesystemResponse::Read {
            content: "".to_string(),
            encoding: "utf-8".to_string(),
        })
    }

    async fn handle_write(
        &self,
        _path: String,
        _content: String,
        _append: bool,
    ) -> Result<FilesystemResponse, anyhow::Error> {
        // This is a placeholder - actual implementation would write to the file
        Ok(FilesystemResponse::Write { bytes_written: 0 })
    }

    async fn handle_delete(
        &self,
        _path: String,
        _recursive: bool,
    ) -> Result<FilesystemResponse, anyhow::Error> {
        // This is a placeholder - actual implementation would delete the file/directory
        Ok(FilesystemResponse::Delete { success: true })
    }
}

// Helper function to create an error response for the appropriate command type
fn create_error_response(_error: &str) -> FilesystemResponse {
    // In a real implementation, we would choose the appropriate response type
    // based on the command. For now, we'll use a list response.
    FilesystemResponse::List { files: vec![] }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::block_on;

    #[test]
    fn test_server_creation() {
        let server = FilesystemServer::new(".");
        assert!(!server.id().is_empty());
    }

    #[test]
    fn test_handle_list_request() {
        let server = FilesystemServer::new(".");

        let request = McpRequest::new(
            "filesystem.list",
            FilesystemCommand::List {
                path: ".".to_string(),
                recursive: false,
            },
        );

        let response = block_on(server.handle_request(request));
        assert!(response.is_ok());

        let response = response.unwrap();
        assert!(response.status.is_success());

        if let FilesystemResponse::List { files } = response.payload {
            // Just checking that we got a list response
            assert!(files.is_empty()); // Our placeholder returns empty list
        } else {
            panic!("Expected List response");
        }
    }
}
