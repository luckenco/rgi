use crate::{Complete, ModelRequest};
use reqwest::header::HeaderMap;

pub mod anthropic;

pub trait LLMProvider: Sized {
    type Response;

    fn base_url(&self) -> &str;
    fn headers(&self) -> HeaderMap;

    // TODO: Use a future and pin the return
    async fn chat_completion(
        &self,
        request: ModelRequest<Complete, Self>,
    ) -> Result<Self::Response, Box<dyn std::error::Error>>;
}
