use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelsResponse {
    pub data: Vec<Model>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "response_token/s")]
    pub response_tokens_per_second: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "prompt_token/s")]
    pub prompt_tokens_per_second: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_eval_count: Option<i32>,
    pub prompt_tokens: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_eval_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval_count: Option<i32>,
    pub completion_tokens: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_total: Option<String>,
    pub total_tokens: i32,
    pub completion_tokens_details: CompletionTokensDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionTokensDetails {
    pub reasoning_tokens: i32,
    pub accepted_prediction_tokens: i32,
    pub rejected_prediction_tokens: i32,
}
