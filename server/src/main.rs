use axum::{extract::{State, ws::{Message, WebSocket, WebSocketUpgrade}}, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::{env, net::SocketAddr};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[derive(Clone)]
struct AppState { db: MySqlPool }

#[derive(Serialize)]
struct Health { status: &'static str, service: &'static str, database: &'static str }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().with_env_filter(
        env::var("RUST_LOG").unwrap_or_else(|_| "info".into())
    ).init();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL is required; copy .env.example to .env and configure MySQL");
    let db = MySqlPoolOptions::new().max_connections(10).connect(&database_url).await?;
    sqlx::migrate!("./migrations").run(&db).await?;
    let state = AppState { db };
    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/universe", get(universe_info))
        .route("/api/v1/session", get(ws_upgrade))
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());
    let addr: SocketAddr = env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".into()).parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("universe server listening on {addr}");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health(State(state): State<AppState>) -> impl IntoResponse {
    let database = if sqlx::query("SELECT 1").execute(&state.db).await.is_ok() { "connected" } else { "unavailable" };
    let code = if database == "connected" { StatusCode::OK } else { StatusCode::SERVICE_UNAVAILABLE };
    (code, Json(Health { status: if database == "connected" { "ok" } else { "degraded" }, service: "world-map-universe-server", database }))
}

#[derive(Serialize)]
struct UniverseInfo { name: &'static str, quadrants: u8, sectors_per_quadrant: u16, model: &'static str }

async fn universe_info() -> Json<UniverseInfo> {
    Json(UniverseInfo { name: "Procedural Universe", quadrants: 4, sectors_per_quadrant: 499, model: "seeded-procedural-prototype" })
}

async fn ws_upgrade(ws: WebSocketUpgrade, State(_state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    // Prototype transport only. Production sessions must authenticate the player,
    // validate message schemas/rates, and route commands through authoritative game state.
    while let Some(Ok(message)) = socket.recv().await {
        match message {
            Message::Ping(bytes) => if socket.send(Message::Pong(bytes)).await.is_err() { break; },
            Message::Text(text) => {
                let response = serde_json::json!({
                    "type": "server_notice",
                    "message": "WebSocket transport is connected; authenticated game commands are not implemented yet.",
                    "received_type": "text",
                    "received_bytes": text.len()
                });
                if socket.send(Message::Text(response.to_string().into())).await.is_err() { break; }
            },
            Message::Close(_) => break,
            _ => {}
        }
    }
}
