//! MCP server registry and management
//!
//! This module provides the server registry and management functionality for MCP servers.

use raco_core::error::CoreError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Server registry for tracking MCP servers
#[derive(Debug)]
pub struct ServerRegistry {
    /// Map of server ID to server info
    servers: Arc<RwLock<HashMap<Uuid, ServerInfo>>>,
}

/// Information about an MCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    /// Server ID
    pub id: Uuid,

    /// Server name
    pub name: String,

    /// Server type
    pub server_type: String,

    /// Server URI
    pub uri: String,

    /// Whether the server is currently active
    #[serde(default)]
    pub active: bool,
}

impl ServerRegistry {
    /// Create a new server registry
    pub fn new() -> Self {
        info!("Creating new MCP server registry");
        Self {
            servers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a new MCP server
    pub async fn register_server(&self, info: ServerInfo) -> Result<(), CoreError> {
        info!("Registering MCP server: {} ({})", info.name, info.id);
        let mut servers = self.servers.write().await;
        if servers.contains_key(&info.id) {
            warn!("Server with ID {} already exists, replacing", info.id);
        }
        servers.insert(info.id, info);
        Ok(())
    }

    /// Unregister an MCP server
    pub async fn unregister_server(&self, id: Uuid) -> Result<(), CoreError> {
        info!("Unregistering MCP server with ID: {}", id);
        let mut servers = self.servers.write().await;
        if servers.remove(&id).is_none() {
            warn!("Server with ID {} not found", id);
        }
        Ok(())
    }

    /// Get information about an MCP server
    pub async fn get_server(&self, id: Uuid) -> Option<ServerInfo> {
        debug!("Getting server info for ID: {}", id);
        let servers = self.servers.read().await;
        servers.get(&id).cloned()
    }

    /// Get information about all MCP servers
    pub async fn get_all_servers(&self) -> Vec<ServerInfo> {
        debug!("Getting info for all servers");
        let servers = self.servers.read().await;
        servers.values().cloned().collect()
    }

    /// Activate an MCP server
    pub async fn activate_server(&self, id: Uuid) -> Result<(), CoreError> {
        info!("Activating MCP server: {}", id);
        let mut servers = self.servers.write().await;
        if let Some(server) = servers.get_mut(&id) {
            server.active = true;
            Ok(())
        } else {
            warn!("Server with ID {} not found", id);
            Err(CoreError::Other(format!("Server with ID {} not found", id)))
        }
    }

    /// Deactivate an MCP server
    pub async fn deactivate_server(&self, id: Uuid) -> Result<(), CoreError> {
        info!("Deactivating MCP server: {}", id);
        let mut servers = self.servers.write().await;
        if let Some(server) = servers.get_mut(&id) {
            server.active = false;
            Ok(())
        } else {
            warn!("Server with ID {} not found", id);
            Err(CoreError::Other(format!("Server with ID {} not found", id)))
        }
    }
}

impl Default for ServerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_server() -> ServerInfo {
        ServerInfo {
            id: Uuid::new_v4(),
            name: "Test Server".to_string(),
            server_type: "test".to_string(),
            uri: "localhost:8080".to_string(),
            active: false,
        }
    }

    #[tokio::test]
    async fn test_register_server() {
        let registry = ServerRegistry::new();
        let server = create_test_server();
        assert!(registry.register_server(server).await.is_ok());
    }

    #[tokio::test]
    async fn test_get_server() {
        let registry = ServerRegistry::new();
        let server = create_test_server();
        let id = server.id;
        let _ = registry.register_server(server.clone()).await;

        let result = registry.get_server(id).await;
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, server.name);
    }

    #[tokio::test]
    async fn test_unregister_server() {
        let registry = ServerRegistry::new();
        let server = create_test_server();
        let id = server.id;
        let _ = registry.register_server(server).await;

        assert!(registry.unregister_server(id).await.is_ok());
        assert!(registry.get_server(id).await.is_none());
    }

    #[tokio::test]
    async fn test_activate_deactivate_server() {
        let registry = ServerRegistry::new();
        let server = create_test_server();
        let id = server.id;
        let _ = registry.register_server(server).await;

        assert!(registry.activate_server(id).await.is_ok());
        let activated = registry.get_server(id).await.unwrap();
        assert!(activated.active);

        assert!(registry.deactivate_server(id).await.is_ok());
        let deactivated = registry.get_server(id).await.unwrap();
        assert!(!deactivated.active);
    }
}
