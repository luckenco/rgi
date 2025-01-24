use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub mod providers;

// Core types
pub struct Incomplete;
pub struct Complete;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Clone)]
pub struct Model<S, P> {
    pub model: String,
    pub max_tokens: i32,
    pub system_prompt: Option<String>,
    pub _state: PhantomData<S>,
    pub _provider: PhantomData<P>,
}

#[derive(Serialize)]
pub struct ModelRequest<S, P> {
    pub model: String,
    pub max_tokens: i32,
    pub messages: Vec<Message>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip)]
    pub _state: PhantomData<S>,
    #[serde(skip)]
    pub _provider: PhantomData<P>,
}

#[derive(Debug, Deserialize)]
pub struct TokenUsage {
    pub input_tokens: i64,
    pub output_tokens: i64,
}

pub use providers::anthropic::{AnthropicClient, AnthropicConfig};
