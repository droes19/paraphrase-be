use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ParaphraseRequest {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct ParaphraseResponse {
    #[serde(rename = "paraphrasedText")]
    pub paraphrased_text: String,
}
