use std::marker::PhantomData;

use crate::{Complete, Incomplete, Message, Model, ModelRequest, TokenUsage};

use super::LLMProvider;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client as HttpClient,
};
use serde::Deserialize;

pub struct DeepSeekConfig;

pub struct DeepSeekClient {
    http_client: HttpClient,
    api_key: String,
    base_url: String,
}

impl DeepSeekClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            http_client: HttpClient::new(),
            api_key: api_key.into(),
            base_url: "http://api.deepseek.com".to_string(),
        }
    }

    pub async fn chat_completion(
        &self,
        request: ModelRequest<Complete, DeepSeekConfig>,
    ) -> Result<DeepSeekResponse, Box<dyn std::error::Error>> {
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

impl LLMProvider for DeepSeekClient {
    type Response = DeepSeekResponse;

    fn base_url(&self) -> &str {
        &self.base_url
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", self.api_key)).unwrap(),
        );
        headers.insert("content-type", HeaderValue::from_static("application/json"));
        headers
    }

    async fn chat_completion(
        &self,
        request: ModelRequest<Complete, Self>,
    ) -> Result<Self::Response, Box<dyn std::error::Error>> {
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
    }
}

#[derive(Debug, Deserialize)]
pub struct Content {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct DeepSeekResponse {
    pub content: Vec<Content>,
    pub usage: TokenUsage,
}

impl DeepSeekResponse {
    pub fn get_text(&self) -> Option<&str> {
        self.content.first().map(|c| c.text.as_str())
    }
}

impl Model<Incomplete, DeepSeekConfig> {
    pub fn deepseek(model: impl Into<String>, max_tokens: i32) -> Self {
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

    pub fn request(&self) -> ModelRequest<Incomplete, DeepSeekConfig> {
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

impl ModelRequest<Incomplete, DeepSeekConfig> {
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

    pub fn build(self) -> Result<ModelRequest<Complete, DeepSeekConfig>, &'static str> {
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
