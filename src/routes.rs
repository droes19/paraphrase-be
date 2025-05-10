use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

use crate::models::{ParaphraseRequest, ParaphraseResponse};
use crate::services::ai::paraphrase_with_ai;
use crate::error::ApiError;

// Health check endpoint
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({ "status": "ok" })))
}

// Paraphrase endpoint
pub async fn paraphrase_text(
    Json(request): Json<ParaphraseRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate the request
    if request.text.trim().is_empty() {
        return Err(ApiError::InvalidInput("Text cannot be empty".to_string()));
    }
    
    // Call AI service to paraphrase the text
    let paraphrased_text = paraphrase_with_ai(&request.text).await?;
    
    // Return the paraphrased text
    let response = ParaphraseResponse {
        paraphrased_text,
    };
    
    Ok((StatusCode::OK, Json(response)))
}
