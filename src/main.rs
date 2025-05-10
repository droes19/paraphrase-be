mod error;
mod models;
mod routes;
mod services;

use axum::{
    routing::{get, post},
    Router,
};
use std::env;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // Initialize environment variables
    dotenv::dotenv().ok();
    
    // Initialize logger
    tracing_subscriber::fmt::init();

    // Get the frontend URL from environment variable or default to localhost
    let frontend_url = env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());
    
    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(frontend_url.parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
        ])
        .allow_credentials(true);
    
    // Application router
    let app = Router::new()
        .route("/", get(routes::health_check))
        .route("/api/paraphrase", post(routes::paraphrase_text))
        .layer(cors);
    
    // Get port from environment variable or use default
    let port = env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Server listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}