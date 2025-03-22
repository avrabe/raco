//! Server registry
//!
//! This module provides a registry for managing MCP servers.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::{ServerError, ServerResult};

/// Information about a registered server
#[derive(Debug, Clone)]
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
    pub active: bool,

    /// Server metadata
    pub metadata: HashMap<String, String>,
}

/// Server registry for managing MCP servers
#[derive(Debug, Clone)]
pub struct ServerRegistry {
    /// Map of server ID to server info
    servers: Arc<RwLock<HashMap<Uuid, ServerInfo>>>,
}

impl ServerRegistry {
    /// Create a new server registry
    pub fn new() -> Self {
        info!("Creating new server registry");
        Self {
            servers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a new server
    pub async fn register_server(&self, info: ServerInfo) -> ServerResult<()> {
        info!("Registering server: {} ({})", info.name, info.id);
        let mut servers = self.servers.write().await;
        if servers.contains_key(&info.id) {
            warn!("Server with ID {} already exists, replacing", info.id);
        }
        servers.insert(info.id, info);
        Ok(())
    }

    /// Unregister a server
    pub async fn unregister_server(&self, id: Uuid) -> ServerResult<()> {
        info!("Unregistering server with ID: {}", id);
        let mut servers = self.servers.write().await;
        if servers.remove(&id).is_none() {
            warn!("Server with ID {} not found", id);
        }
        Ok(())
    }

    /// Get information about a server
    pub async fn get_server(&self, id: Uuid) -> ServerResult<Option<ServerInfo>> {
        debug!("Getting server info for ID: {}", id);
        let servers = self.servers.read().await;
        Ok(servers.get(&id).cloned())
    }

    /// Get information about all servers
    pub async fn get_all_servers(&self) -> ServerResult<Vec<ServerInfo>> {
        debug!("Getting info for all servers");
        let servers = self.servers.read().await;
        Ok(servers.values().cloned().collect())
    }

    /// Get information about servers of a specific type
    pub async fn get_servers_by_type(&self, server_type: &str) -> ServerResult<Vec<ServerInfo>> {
        debug!("Getting servers of type: {}", server_type);
        let servers = self.servers.read().await;
        Ok(servers
            .values()
            .filter(|s| s.server_type == server_type)
            .cloned()
            .collect())
    }

    /// Find a server by name
    pub async fn find_server_by_name(&self, name: &str) -> ServerResult<Option<ServerInfo>> {
        debug!("Finding server by name: {}", name);
        let servers = self.servers.read().await;
        Ok(servers.values().find(|s| s.name == name).cloned())
    }

    /// Activate a server
    pub async fn activate_server(&self, id: Uuid) -> ServerResult<()> {
        info!("Activating server: {}", id);
        let mut servers = self.servers.write().await;
        if let Some(server) = servers.get_mut(&id) {
            server.active = true;
            Ok(())
        } else {
            warn!("Server with ID {} not found", id);
            Err(ServerError::ServerNotFound(id.to_string()))
        }
    }

    /// Deactivate a server
    pub async fn deactivate_server(&self, id: Uuid) -> ServerResult<()> {
        info!("Deactivating server: {}", id);
        let mut servers = self.servers.write().await;
        if let Some(server) = servers.get_mut(&id) {
            server.active = false;
            Ok(())
        } else {
            warn!("Server with ID {} not found", id);
            Err(ServerError::ServerNotFound(id.to_string()))
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
            metadata: HashMap::new(),
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

        let result = registry.get_server(id).await.unwrap();
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
        assert!(registry.get_server(id).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_activate_deactivate_server() {
        let registry = ServerRegistry::new();
        let server = create_test_server();
        let id = server.id;

        let _ = registry.register_server(server).await;

        assert!(registry.activate_server(id).await.is_ok());
        let activated = registry.get_server(id).await.unwrap().unwrap();
        assert!(activated.active);

        assert!(registry.deactivate_server(id).await.is_ok());
        let deactivated = registry.get_server(id).await.unwrap().unwrap();
        assert!(!deactivated.active);
    }
}
