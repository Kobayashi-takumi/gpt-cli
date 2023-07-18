use serde::{Deserialize, Serialize};

//
// Chat
//
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    pub fn send(value: String) -> Self {
        Self {
            role: "user".to_string(),
            content: value,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub usage: Usage,
    pub choices: Vec<Choice>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub message: Message,
    pub finish_reason: String,
    pub index: u64,
}

//
// Image
//
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImageRequest {
    pub prompt: String,
    pub n: i64,
    pub size: String,
}

impl ImageRequest {
    pub fn sender(value: String) -> Self {
        Self {
            prompt: value,
            n: 1,
            size: "1024x1024".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImageResponse {
    pub created: u64,
    pub data: Vec<Url>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Url {
    pub url: String,
}

//
// Error
//
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorContent,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorContent {
    pub code: String,
    pub message: String,
    #[serde(rename = "type")]
    pub type_: String,
}
