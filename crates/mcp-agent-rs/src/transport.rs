// Transport module for mcp-agent-rs
#![allow(dead_code)]

use anyhow::Result;
use std::fmt::Debug;

/// Transport trait for MCP communication
pub trait Transport: Debug + Send + Sync {
    /// Send a message
    fn send(&self, _message: &str) -> Result<()> {
        Ok(())
    }

    /// Receive a message
    fn receive(&self) -> Result<String> {
        Ok("".to_string())
    }
}

/// Mock transport for testing
#[derive(Debug)]
pub struct MockTransport {
    /// Whether the transport is auto-connect
    auto_connect: bool,
}

impl Default for MockTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl MockTransport {
    /// Create a new mock transport
    pub fn new() -> Self {
        Self {
            auto_connect: false,
        }
    }

    /// Create a new mock transport with auto-connect
    pub fn new_auto_connect() -> Self {
        Self { auto_connect: true }
    }
}

impl Transport for MockTransport {}

/// Stdio transport
#[cfg(feature = "transport-stdio")]
#[derive(Debug)]
pub struct StdioTransport;

#[cfg(feature = "transport-stdio")]
impl Default for StdioTransport {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "transport-stdio")]
impl StdioTransport {
    /// Create a new stdio transport
    pub fn new() -> Self {
        Self
    }
}

#[cfg(feature = "transport-stdio")]
impl Transport for StdioTransport {}

/// WebSocket transport
#[cfg(feature = "transport-websocket")]
#[derive(Debug)]
pub struct WebSocketTransport {
    /// WebSocket URL
    url: String,
}

#[cfg(feature = "transport-websocket")]
impl WebSocketTransport {
    /// Create a new WebSocket transport
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }
}

#[cfg(feature = "transport-websocket")]
impl Transport for WebSocketTransport {}
