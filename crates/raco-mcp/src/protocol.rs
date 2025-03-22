//! MCP protocol implementation
//!
//! This module provides the protocol implementation for the Model Context Protocol.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MCP command types supported by RACO
pub enum CommandType {
    /// Execute a command
    Execute,

    /// Query the system for information
    Query,

    /// Monitor the system for events
    Monitor,

    /// Register a new component
    Register,
}

impl CommandType {
    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Execute => "execute",
            Self::Query => "query",
            Self::Monitor => "monitor",
            Self::Register => "register",
        }
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "execute" => Some(Self::Execute),
            "query" => Some(Self::Query),
            "monitor" => Some(Self::Monitor),
            "register" => Some(Self::Register),
            _ => None,
        }
    }
}

/// Generic MCP request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpRequest<T> {
    /// Command type
    pub command: String,

    /// Payload
    pub payload: T,

    /// Request ID for tracking
    #[serde(default)]
    pub request_id: Option<String>,
}

impl<T> McpRequest<T> {
    /// Create a new MCP request
    pub fn new(command: &str, payload: T) -> Self {
        Self {
            command: command.to_string(),
            payload,
            request_id: Some(uuid::Uuid::new_v4().to_string()),
        }
    }

    /// Create a new MCP request from a command type
    pub fn from_command_type(command_type: CommandType, payload: T) -> Self {
        Self::new(command_type.as_str(), payload)
    }
}

/// Generic MCP response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResponse<T> {
    /// Command type
    pub command: String,

    /// Payload
    pub payload: T,

    /// Response status
    pub status: ResponseStatus,

    /// Request ID for tracking (matches the request)
    pub request_id: Option<String>,
}

/// Response status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseStatus {
    /// Status code (0 for success, non-zero for error)
    pub code: i32,

    /// Status message
    pub message: String,
}

impl ResponseStatus {
    /// Create a success status
    pub fn success() -> Self {
        Self {
            code: 0,
            message: "Success".to_string(),
        }
    }

    /// Create an error status
    pub fn error(code: i32, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
        }
    }

    /// Check if the status indicates success
    pub fn is_success(&self) -> bool {
        self.code == 0
    }
}

/// File system information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// File name
    pub name: String,

    /// File path
    pub path: String,

    /// File size in bytes
    pub size: u64,

    /// Whether the file is a directory
    pub is_directory: bool,

    /// File metadata
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

/// Process information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    /// Process ID
    pub pid: u32,

    /// Process name
    pub name: String,

    /// Command used to start the process
    pub command: String,

    /// Process status
    pub status: String,

    /// Process metadata
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_type_conversion() {
        let cmd = CommandType::Execute;
        assert_eq!(cmd.as_str(), "execute");
        assert_eq!(
            CommandType::from_str("execute").unwrap().as_str(),
            "execute"
        );
        assert!(CommandType::from_str("invalid").is_none());
    }

    #[test]
    fn test_mcp_request_serialization() {
        let request = McpRequest::new("test", "payload");
        let json = serde_json::to_string(&request).unwrap();
        let deserialized: McpRequest<String> = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.command, "test");
        assert_eq!(deserialized.payload, "payload");
        assert!(deserialized.request_id.is_some());
    }

    #[test]
    fn test_response_status() {
        let success = ResponseStatus::success();
        assert!(success.is_success());

        let error = ResponseStatus::error(1, "Error message");
        assert!(!error.is_success());
        assert_eq!(error.code, 1);
        assert_eq!(error.message, "Error message");
    }
}
