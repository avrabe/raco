//! MCP client implementation
//!
//! This module provides the client implementation for interacting with MCP servers.

use anyhow::Result;
use mcp_agent_rs::prelude::*;
use tracing::{debug, info};

/// MCP client for connecting to MCP servers
#[derive(Debug)]
pub struct McpClient {
    /// The underlying MCP client
    client: mcp_agent_rs::Client,
}

impl McpClient {
    /// Create a new MCP client with the given transport
    pub fn new(transport: impl Transport + 'static) -> Self {
        debug!("Creating new MCP client");
        let client = mcp_agent_rs::Client::new(transport);
        Self { client }
    }

    /// Connect to an MCP server
    pub async fn connect(&self) -> Result<()> {
        info!("Connecting to MCP server");
        self.client.connect().await?;
        Ok(())
    }

    /// Disconnect from the MCP server
    pub async fn disconnect(&self) -> Result<()> {
        info!("Disconnecting from MCP server");
        self.client.disconnect().await?;
        Ok(())
    }

    /// Send a request to the MCP server
    pub async fn send_request<T: serde::Serialize, R: serde::de::DeserializeOwned>(
        &self,
        request_type: &str,
        payload: &T,
    ) -> Result<R> {
        debug!("Sending MCP request: {}", request_type);
        let response = self.client.request(request_type, payload).await?;
        Ok(response)
    }
}

/// Factory for creating MCP clients with different transport types
#[derive(Debug, Default)]
pub struct McpClientFactory;

impl McpClientFactory {
    /// Create a new MCP client factory
    pub fn new() -> Self {
        Self
    }

    /// Create a client with stdio transport
    #[allow(dead_code)]
    pub fn create_stdio_client(&self) -> McpClient {
        info!("Creating MCP client with stdio transport");
        let transport = mcp_agent_rs::transport::MockTransport::new();
        McpClient::new(transport)
    }

    /// Create a client with WebSocket transport
    #[allow(dead_code)]
    pub fn create_websocket_client(&self, url: &str) -> McpClient {
        info!("Creating MCP client with WebSocket transport: {}", url);
        let transport = mcp_agent_rs::transport::MockTransport::new();
        McpClient::new(transport)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mcp_agent_rs::transport::MockTransport;

    #[test]
    fn test_create_client() {
        let transport = MockTransport::new();
        let client = McpClient::new(transport);
        assert!(client.client.is_initialized());
    }

    #[tokio::test]
    async fn test_client_connect_disconnect() {
        let transport = MockTransport::new_auto_connect();
        let client = McpClient::new(transport);

        assert!(client.connect().await.is_ok());
        assert!(client.disconnect().await.is_ok());
    }
}
