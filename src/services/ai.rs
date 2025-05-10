use reqwest::Client;
use std::env;

use crate::error::ApiError;

// Default Anthropic API URL
const DEFAULT_AI_API_URL: &str = "https://api.anthropic.com/v1/messages";

pub async fn paraphrase_with_ai(text: &str) -> Result<String, ApiError> {
    // Get API key from environment variable
    let api_key = env::var("AI_API_KEY")
        .map_err(|_| ApiError::InternalServerError("AI API key not configured".to_string()))?;
    
    // Create reqwest client
    let client = Client::new();
    
    // Get the API URL (default to Anthropic API)
    let ai_url = env::var("AI_API_URL").unwrap_or_else(|_| DEFAULT_AI_API_URL.to_string());
    
    // Define the system prompt to instruct the AI
    let system_prompt = "You are an expert at paraphrasing text. Your task is to rewrite the provided text with different words and structure while preserving the original meaning. Avoid simply replacing words with synonyms. Instead, restructure sentences and use different phrases to express the same ideas. The paraphrased text should be natural, fluent, and maintain the same tone and style as the original.";
    
    // Call the Anthropic API
    let response = client
        .post(&ai_url)
        .header("x-api-key", &api_key)
        .header("anthropic-version", "2023-06-01")
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "model": "claude-3-haiku-20240307",
            "system": system_prompt,
            "messages": [
                {
                    "role": "user",
                    "content": format!("Please paraphrase the following text. Only return the paraphrased version, no explanations or additional text: {}", text)
                }
            ],
            "max_tokens": 1024,
            "temperature": 0.7
        }))
        .send()
        .await
        .map_err(|e| ApiError::AiServiceError(format!("Failed to call Anthropic API: {}", e)))?;
    
    // Check if the response is successful
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(ApiError::AiServiceError(format!("Anthropic API returned an error: {}", error_text)));
    }
    
    // Parse the response for Anthropic's format
    let response_json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| ApiError::AiServiceError(format!("Failed to parse Anthropic API response: {}", e)))?;
    
    // Extract the content from the response
    let content = response_json
        .get("content")
        .and_then(|content| content.as_array())
        .and_then(|content_array| content_array.get(0))
        .and_then(|content_item| content_item.get("text"))
        .and_then(|text| text.as_str())
        .ok_or_else(|| ApiError::AiServiceError("Anthropic API returned an invalid response format".to_string()))?;
    
    Ok(content.trim().to_string())
}

// Fallback function for development/testing without an actual AI API key
#[allow(dead_code)]
async fn mock_paraphrase(text: &str) -> Result<String, ApiError> {
    // Simple mock transformation for testing
    let words: Vec<&str> = text.split_whitespace().collect();
    let paraphrased: Vec<String> = words
        .iter()
        .map(|&word| {
            if word.len() > 3 && rand::random::<bool>() {
                let synonym = get_mock_synonym(word);
                synonym.to_string()
            } else {
                word.to_string()
            }
        })
        .collect();
    
    Ok(paraphrased.join(" "))
}

#[allow(dead_code)]
fn get_mock_synonym(word: &str) -> &str {
    // Extremely simplified "synonym" generator for testing
    match word.to_lowercase().as_str() {
        "good" => "excellent",
        "bad" => "poor",
        "big" => "large",
        "small" => "tiny",
        "happy" => "joyful",
        "sad" => "unhappy",
        "quick" => "fast",
        "slow" => "sluggish",
        _ => word,
    }
}
