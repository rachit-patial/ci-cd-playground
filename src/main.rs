use axum::{
    extract::Query,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Deserialize)]
struct SearchQuery {
    q: Option<String>,
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: Option<String>,
}

#[derive(Serialize)]
struct ApiResponse {
    status: String,
    message: String,
}

#[tokio::main]
async fn main() {
    // Initialize logging output for SAST/logging checks
    tracing_subscriber::fmt::init();

    // Define application routes
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_check))
        .route("/api/search", get(search_handler))
        .route("/api/login", post(login_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Basic root endpoint
async fn root_handler() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
    
    (
        StatusCode::OK,
        headers,
        "<h1>Rust Application API</h1><p>Ready for DAST scanning.</p>",
    )
}

// Health check endpoint for uptime/readiness probes
async fn health_check() -> impl IntoResponse {
    Json(ApiResponse {
        status: "UP".to_string(),
        message: "Service running normally".to_string(),
    })
}

// Endpoint with Query parameters (ideal target for DAST fuzzing/injection testing)
async fn search_handler(Query(query): Query<SearchQuery>) -> impl IntoResponse {
    let search_term = query.q.unwrap_or_else(|| "default".to_string());

    Json(ApiResponse {
        status: "success".to_string(),
        message: format!("Results for search term: {}", search_term),
    })
}

// Endpoint accepting POST payload
async fn login_handler(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    if payload.username.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                status: "error".to_string(),
                message: "Username cannot be empty".to_string(),
            }),
        );
    }

    (
        StatusCode::OK,
        Json(ApiResponse {
            status: "success".to_string(),
            message: format!("User '{}' processed", payload.username),
        }),
    )
}