use reqwest::{header, Client as HttpClient};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use serde_json::json;
use std::time::Duration;

pub mod completion;
pub mod request;

pub const MODEL: &str = "deepseek-reasoner";

pub struct Config {
    pub base_url: &'static str,
    pub model: &'static str,
    pub connection_timeout: Duration,
    pub max_retries: u32,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_url: "https://api.deepseek.com",
            model: MODEL,
            max_retries: 3,
            // Keep in mind that this should be way lower when streaming completion chunks.
            connection_timeout: Duration::from_secs(300),
        }
    }
}

pub struct Client {
    pub inner: ClientWithMiddleware,
    config: Config,
}

impl Client {
    pub fn new(api_key: &str, config: Config) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());
        headers.insert(
            "authorization",
            format!("Bearer {}", api_key).parse().unwrap(),
        );

        let http_client = HttpClient::builder()
            .timeout(config.connection_timeout)
            .default_headers(headers)
            .build()
            .unwrap();

        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(config.max_retries);

        let retry_client = ClientBuilder::new(http_client)
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        Self {
            inner: retry_client,
            config,
        }
    }

    pub async fn complete(
        &self,
        request: request::Chat,
    ) -> Result<completion::Object, Box<dyn std::error::Error>> {
        let request_url = format!("{}/chat/completions", self.config.base_url);

        let body = json!(request);
        println!("Request: {}", body);

        let response = self
            .inner
            .post(request_url)
            .body(body.to_string())
            .send()
            .await?
            .json::<completion::Object>()
            .await?;

        println!("Response: {:?}", response);

        Ok(response)
    }
}
