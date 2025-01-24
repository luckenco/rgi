use std::future::Future;
use std::pin::Pin;

use crate::{Complete, ModelRequest};
use reqwest::header::HeaderMap;

pub mod anthropic;

pub trait LLMProvider: Sized {
    type Response;

    fn base_url(&self) -> &str;
    fn headers(&self) -> HeaderMap;

    fn chat_completion(
        &self,
        request: ModelRequest<Complete, Self>,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Response, Box<dyn std::error::Error>>> + Send + '_>>;
}
