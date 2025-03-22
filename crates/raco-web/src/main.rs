// RACO Web - Web interface for RACO

use anyhow::{Context, Result};
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use raco_core::config::load_config;
use raco_core::utils::ensure_dir_exists;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{debug, info, warn};
use tracing_subscriber::EnvFilter;

/// Application state
#[derive(Debug, Clone)]
struct AppState {
    #[allow(dead_code)]
    config: Arc<raco_core::config::CoreConfig>,
    server_registry: Arc<RwLock<raco_servers::registry::ServerRegistry>>,
}

/// Server info response
#[derive(Debug, Serialize, Deserialize)]
struct ServerInfoResponse {
    id: String,
    name: String,
    server_type: String,
    uri: String,
    active: bool,
}

/// API error response
#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

/// Initialize logging
fn init_logging() {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,raco_web=debug"));

    let _ = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(true)
        .try_init();
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    init_logging();

    info!("Starting RACO Web server");

    // Load configuration
    let config = load_config().context("Failed to load configuration")?;

    // Ensure data directory exists
    ensure_dir_exists(&config.data_dir).context("Failed to create data directory")?;

    debug!("Using data directory: {}", config.data_dir.display());

    // Create application state
    let app_state = AppState {
        config: Arc::new(config),
        server_registry: Arc::new(RwLock::new(raco_servers::registry::ServerRegistry::new())),
    };

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build application router
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/api/servers", get(list_servers))
        .route("/api/servers", post(register_server))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(app_state);

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("Failed to start server")?;

    Ok(())
}

/// Root handler
async fn root_handler() -> impl IntoResponse {
    "RACO Web API"
}

/// List servers handler
async fn list_servers(
    State(state): State<AppState>,
) -> Result<Json<Vec<ServerInfoResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let registry = state.server_registry.read().await;

    match registry.get_all_servers().await {
        Ok(servers) => {
            let response = servers
                .into_iter()
                .map(|s| ServerInfoResponse {
                    id: s.id.to_string(),
                    name: s.name,
                    server_type: s.server_type,
                    uri: s.uri,
                    active: s.active,
                })
                .collect();

            Ok(Json(response))
        }
        Err(e) => {
            warn!("Error listing servers: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to list servers: {}", e),
                }),
            ))
        }
    }
}

/// Register server request
#[derive(Debug, Deserialize)]
struct RegisterServerRequest {
    name: String,
    server_type: String,
    uri: String,
}

/// Register server handler
async fn register_server(
    State(state): State<AppState>,
    Json(request): Json<RegisterServerRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let registry = state.server_registry.write().await;

    let server_info = raco_servers::registry::ServerInfo {
        id: uuid::Uuid::new_v4(),
        name: request.name,
        server_type: request.server_type,
        uri: request.uri,
        active: false,
        metadata: std::collections::HashMap::new(),
    };

    match registry.register_server(server_info).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            warn!("Error registering server: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to register server: {}", e),
                }),
            ))
        }
    }
}
