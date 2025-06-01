use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelsResponse {
    pub data: Vec<Model>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub name: String,
    pub object: String,
    pub created: i64,
    pub owned_by: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ollama: Option<Ollama>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<Info>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arena: Option<bool>,
    pub tags: Vec<String>,
    pub actions: Vec<serde_json::Value>, // Using Value since the arrays are empty
    pub filters: Vec<serde_json::Value>, // Using Value since the arrays are empty
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ollama {
    pub name: String,
    pub model: String,
    pub modified_at: String,
    pub size: i64,
    pub digest: String,
    pub details: Details,
    pub connection_type: String,
    pub urls: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Details {
    pub parent_model: String,
    pub format: String,
    pub family: String,
    pub families: Vec<String>,
    pub parameter_size: String,
    pub quantization_level: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub meta: Meta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub profile_image_url: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_ids: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub object: String,
    pub usage: Usage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub index: i32,
    pub logprobs: Option<serde_json::Value>, // null in the JSON
    pub finish_reason: String,
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub content: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    #[serde(rename = "response_token/s")]
    pub response_tokens_per_second: f64,
    #[serde(rename = "prompt_token/s")]
    pub prompt_tokens_per_second: f64,
    pub total_duration: i64,
    pub load_duration: i64,
    pub prompt_eval_count: i32,
    pub prompt_tokens: i32,
    pub prompt_eval_duration: i64,
    pub eval_count: i32,
    pub completion_tokens: i32,
    pub eval_duration: i64,
    pub approximate_total: String,
    pub total_tokens: i32,
    pub completion_tokens_details: CompletionTokensDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionTokensDetails {
    pub reasoning_tokens: i32,
    pub accepted_prediction_tokens: i32,
    pub rejected_prediction_tokens: i32,
}
