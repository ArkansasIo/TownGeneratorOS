use axum::{extract::{State, ws::{Message, WebSocket, WebSocketUpgrade}}, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::{env, net::SocketAddr};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
mod physics;
mod roads;
mod planet;
use physics::{OrbitRequest, SurfaceGravityRequest};

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
        .route("/api/v1/physics/orbit", axum::routing::post(orbit_calculation))
        .route("/api/v1/physics/surface-gravity", axum::routing::post(surface_gravity_calculation))
        .route("/api/v1/world/roads", axum::routing::post(generate_roads))
        .route("/api/v1/planets/generate", axum::routing::post(generate_planet))
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

async fn orbit_calculation(Json(input): Json<OrbitRequest>) -> impl IntoResponse {
    match physics::calculate_orbit(&input) {
        Ok(result) => (StatusCode::OK, Json(serde_json::json!({"model":"two-body Keplerian approximation","units":"SI","result":result}))),
        Err(message) => (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error":message}))),
    }
}

async fn surface_gravity_calculation(Json(input): Json<SurfaceGravityRequest>) -> impl IntoResponse {
    match physics::surface_gravity(&input) {
        Ok(result) => (StatusCode::OK, Json(serde_json::json!({"model":"Newtonian spherical-body approximation","units":"SI","result":result}))),
        Err(message) => (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error":message}))),
    }
}

async fn generate_roads(Json(input): Json<roads::RoadNetworkRequest>) -> impl IntoResponse {
    match roads::generate_road_network(input.seed, input.sites, input.max_local_distance) {
        Ok(network) => (StatusCode::OK, Json(serde_json::json!({"algorithm":"deterministic_euclidean_backbone_v1","note":"Prototype graph only; terrain, rivers, walls, slope costs and pathfinding are not yet applied.","network":network}))),
        Err(message) => (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error":message}))),
    }
}

#[derive(serde::Deserialize)]
struct GeneratePlanetRequest { seed: u64, id: String, name: String, class: planet::PlanetClass }

async fn generate_planet(Json(input): Json<GeneratePlanetRequest>) -> impl IntoResponse {
    if input.id.trim().is_empty() || input.name.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error":"id and name must not be empty"})));
    }
    let profile = planet::generate_planet(input.seed, input.id, input.name, input.class);
    (StatusCode::OK, Json(serde_json::json!({"model":"seeded procedural planetary profile v1","warning":"Generated values are fictional game estimates, not measured astronomical data.","planet":profile})))
}
