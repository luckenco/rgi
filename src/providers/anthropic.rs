use crate::{Message, Model};

use super::Provider;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

pub struct Anthropic;

#[derive(Default)]
pub struct AnthropicConfig;

#[derive(Serialize)]
pub struct AnthropicRequest {
    pub model: String,
    pub max_tokens: u32,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
}

impl From<Model<Anthropic>> for AnthropicRequest {
    fn from(model: Model<Anthropic>) -> Self {
        AnthropicRequest {
            model: model.model,
            max_tokens: model.max_tokens,
            system: None,
            messages: Vec::new(),
            temperature: None,
        }
    }
}

#[derive(Deserialize)]
pub struct AnthropicResponse {
    pub content: Vec<Content>,
    pub id: String,
    pub model: String,
    pub role: String,
    pub stop_reason: String,
    pub stop_sequence: Option<String>,
    #[serde(rename = "type")]
    pub response_type: String,
    pub usage: Usage,
}

#[derive(Debug, Deserialize)]
pub struct Content {
    pub text: String,
    #[serde(rename = "type")]
    pub message_type: String,
}

#[derive(Deserialize)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

impl Provider for Anthropic {
    type Config = AnthropicConfig;
    type Request = AnthropicRequest;
    type Response = AnthropicResponse;

    fn base_url() -> &'static str {
        "https://api.anthropic.com/v1/messages"
    }

    fn headers(api_key: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-api-key",
            HeaderValue::from_str(api_key).expect("Invalid API key format"),
        );
        headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));
        headers.insert("content-type", HeaderValue::from_static("application/json"));
        headers
    }
}

pub trait AnthropicRequestBuilder {
    fn system_prompt(self, prompt: impl Into<String>) -> Self;
    fn message(self, role: impl Into<String>, content: impl Into<String>) -> Self;
    fn temperature(self, temp: f32) -> Self;
}

impl AnthropicRequestBuilder for AnthropicRequest {
    fn system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system = Some(prompt.into());
        self
    }

    fn message(mut self, role: impl Into<String>, content: impl Into<String>) -> Self {
        self.messages.push(Message {
            role: role.into(),
            content: content.into(),
        });
        self
    }

    fn temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }
}
