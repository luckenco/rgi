use std::{future::Future, marker::PhantomData, pin::Pin};

use crate::{Complete, Incomplete, Message, Model, ModelRequest, TokenUsage};

use super::LLMProvider;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client as HttpClient,
};
use serde::Deserialize;

pub struct AnthropicConfig;

pub struct AnthropicClient {
    http_client: HttpClient,
    api_key: String,
    base_url: String,
}

impl AnthropicClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            http_client: HttpClient::new(),
            api_key: api_key.into(),
            base_url: "https://api.anthropic.com/v1/messages".to_string(),
        }
    }

    pub async fn chat_completion(
        &self,
        request: ModelRequest<Complete, AnthropicConfig>,
    ) -> Result<AnthropicResponse, Box<dyn std::error::Error>> {
        let response = self
            .http_client
            .post(&self.base_url)
            .headers(self.headers())
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
}

impl LLMProvider for AnthropicClient {
    type Response = AnthropicResponse;

    fn base_url(&self) -> &str {
        &self.base_url
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("x-api-key", HeaderValue::from_str(&self.api_key).unwrap());
        headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));
        headers.insert("content-type", HeaderValue::from_static("application/json"));
        headers
    }

    fn chat_completion(
        &self,
        request: ModelRequest<Complete, Self>,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Response, Box<dyn std::error::Error>>> + Send + '_>>
    {
        Box::pin(async move {
            let response = self
                .http_client
                .post(self.base_url())
                .headers(self.headers())
                .json(&request)
                .send()
                .await?
                .json()
                .await?;

            Ok(response)
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct Content {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct AnthropicResponse {
    pub content: Vec<Content>,
    pub usage: TokenUsage,
}

impl AnthropicResponse {
    pub fn get_text(&self) -> Option<&str> {
        self.content.first().map(|c| c.text.as_str())
    }
}

impl Model<Incomplete, AnthropicConfig> {
    pub fn anthropic(model: impl Into<String>, max_tokens: i32) -> Self {
        Self {
            model: model.into(),
            max_tokens,
            system_prompt: None,
            _state: PhantomData,
            _provider: PhantomData,
        }
    }

    pub fn system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = Some(prompt.into());
        self
    }

    pub fn request(&self) -> ModelRequest<Incomplete, AnthropicConfig> {
        ModelRequest {
            model: self.model.clone(),
            max_tokens: self.max_tokens,
            messages: Vec::new(),
            system: self.system_prompt.clone(),
            temperature: None,
            _state: PhantomData,
            _provider: PhantomData,
        }
    }
}

impl ModelRequest<Incomplete, AnthropicConfig> {
    pub fn temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }

    pub fn message(mut self, role: impl Into<String>, content: impl Into<String>) -> Self {
        self.messages.push(Message {
            role: role.into(),
            content: content.into(),
        });
        self
    }

    pub fn build(self) -> Result<ModelRequest<Complete, AnthropicConfig>, &'static str> {
        if self.messages.is_empty() {
            return Err("messages are required");
        }

        Ok(ModelRequest {
            model: self.model,
            max_tokens: self.max_tokens,
            messages: self.messages,
            system: self.system,
            temperature: self.temperature,
            _state: PhantomData,
            _provider: PhantomData,
        })
    }
}
