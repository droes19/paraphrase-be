mod error;
mod models;
mod routes;
mod services;

use axum::{
    routing::{get, post},
    Router,
};
use shuttle_axum::ShuttleAxum;
use shuttle_runtime::SecretStore;
use sync_wrapper::SyncWrapper;
use tower_http::cors::CorsLayer;
use std::env;

// This is the main entry point for Shuttle
#[shuttle_runtime::main]
async fn axum(
    #[shuttle_secrets::Secrets] secrets: SecretStore,
) -> ShuttleAxum {
    // Initialize logger
    tracing_subscriber::fmt::init();

    // Get secrets from Shuttle
    if let Some(ai_api_key) = secrets.get("AI_API_KEY") {
        env::set_var("AI_API_KEY", ai_api_key);
    }
    
    if let Some(ai_api_url) = secrets.get("AI_API_URL") {
        env::set_var("AI_API_URL", ai_api_url);
    }

    // Development environment - use dotenv
    #[cfg(debug_assertions)]
    {
        dotenv::dotenv().ok();
    }

    // Create app with router
    let app = create_app();

    // Shuttle expects a specific return type
    Ok(SyncWrapper::new(app))
}

// For local development and testing
#[cfg(debug_assertions)]
#[tokio::main]
async fn main() {
    // Initialize environment variables for local development
    dotenv::dotenv().ok();
    
    // Initialize logger
    tracing_subscriber::fmt::init();
    
    // Create app with router
    let app = create_app();
    
    // Start the server
    let addr = "0.0.0.0:8080";
    println!("Server listening on {}", addr);
    
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Shared function to create the app router
fn create_app() -> Router {
    // CORS configuration - allow both localhost and deployed frontend
    let frontend_url = env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    
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
    Router::new()
        .route("/", get(routes::health_check))
        .route("/api/paraphrase", post(routes::paraphrase_text))
        .layer(cors)
}
