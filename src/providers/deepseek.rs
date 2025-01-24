use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::Model;

use super::Provider;

pub struct Deepseek;

#[derive(Default)]
pub struct DeepseekConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Serialize)]
pub struct DeepSeekRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

impl From<Model<Deepseek>> for DeepSeekRequest {
    fn from(model: Model<Deepseek>) -> Self {
        DeepSeekRequest {
            model: model.model,
            max_tokens: Some(model.max_tokens),
            messages: Vec::new(),
            temperature: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepSeekResponse {
    pub id: String,
    pub choices: Vec<Choice>,
    pub created: u64,
    pub model: String,
    pub object: String,
    pub usage: Usage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub finish_reason: String,
    pub index: u32,
    pub message: Message,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub completion_tokens: u32,
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}

impl Provider for Deepseek {
    type Config = DeepseekConfig;
    type Request = DeepSeekRequest;
    type Response = DeepSeekResponse;

    fn base_url() -> &'static str {
        "https://api.deepseek.com/chat/completions"
    }

    fn headers(api_key: &str) -> reqwest::header::HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
        );
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers
    }
}

pub trait DeepSeekRequestBuilder {
    fn system_prompt(self, prompt: impl Into<String>) -> Self;
    fn message(self, role: Role, content: impl Into<String>) -> Self;
    fn temperature(self, temp: f32) -> Self;
}

impl DeepSeekRequestBuilder for DeepSeekRequest {
    fn system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.messages.push(Message {
            role: Role::System,
            content: prompt.into(),
        });
        self
    }

    fn message(mut self, role: Role, content: impl Into<String>) -> Self {
        self.messages.push(Message {
            role,
            content: content.into(),
        });
        self
    }

    fn temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }
}
