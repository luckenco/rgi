use reqwest::header::HeaderMap;
use serde::{de::DeserializeOwned, Serialize};

pub mod anthropic;
pub mod deepseek;

pub trait Provider: Sized {
    type Config: Default;
    type Request: Serialize;
    type Response: DeserializeOwned;

    fn base_url() -> &'static str;
    fn headers(api_key: &str) -> HeaderMap;
}
