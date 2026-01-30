// Dimensionless Developments Rust Ai
// # WebSocket Server Module
// Provides WebSocket server functionality for the frontend to communicate
// with the AI research agent backend.

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::mpsc;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use tracing::{error, info};

use crate::agent::ResearchAgent;
use crate::config::Config;

/// Create the web server router with WebSocket and static file serving
pub fn create_router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let serve_dir = ServeDir::new("dist")
        .append_index_html_on_directories(true)
        .not_found_service(ServeDir::new("dist"));

    Router::new()
        .route("/ws", get(ws_handler))
        .fallback_service(serve_dir)
        .layer(cors)
}

/// WebSocket upgrade handler
async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

/// Handle individual WebSocket connections
async fn handle_socket(socket: WebSocket) {
    info!("New WebSocket connection established");

    let (mut sender, mut receiver) = socket.split();

    // Create a channel for streaming responses
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    // Spawn task to send messages to client
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(query))) = receiver.next().await {
            info!("Received query: {}", query);

            // Load config and create agent
            let config = match Config::from_env() {
                Ok(c) => c,
                Err(e) => {
                    error!("Failed to load config: {}", e);
                    let _ = tx.send(format!("Error: Failed to load configuration - {}", e));
                    continue;
                }
            };

            if let Err(e) = config.validate() {
                error!("Invalid config: {}", e);
                let _ = tx.send(format!("Error: Invalid configuration - {}", e));
                continue;
            }

            let agent = ResearchAgent::new(config);

            // Perform research
            match agent.research(&query).await {
                Ok(response) => {
                    // Send response back to client
                    if tx.send(response).is_err() {
                        error!("Failed to send response to client");
                        break;
                    }
                }
                Err(e) => {
                    error!("Research failed: {}", e);
                    let _ = tx.send(format!("Error: Research failed - {}", e));
                }
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }

    info!("WebSocket connection closed");
}