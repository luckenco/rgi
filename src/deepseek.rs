use completion::Chunk;
use reqwest::{header, Client as HttpClient};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use serde_json::json;
use std::time::Duration;

pub mod completion;
pub mod request;

pub const MODEL: &str = "deepseek/deepseek-r1-distill-llama-70b";

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
            base_url: "https://openrouter.ai",
            model: MODEL,
            max_retries: 3,
            // Keep in mind that this should be way lower when streaming completion chunks.
            connection_timeout: Duration::from_secs(3600),
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

        let response = self
            .inner
            .post(request_url)
            .body(body.to_string())
            .send()
            .await?
            .json::<completion::Object>()
            .await?;

        Ok(response)
    }
}

pub async fn stream(
    client: &Client,
    request: request::Chat,
) -> Result<(), Box<dyn std::error::Error>> {
    let request_url = format!("{}/api/v1/chat/completions", client.config.base_url);

    let body = json!(request);
    println!("Request: {}", body);

    let mut response = client
        .inner
        .post(&request_url)
        .header("accept", "text/event-stream")
        .body(body.to_string())
        .send()
        .await?
        .error_for_status()?;

    let mut buffer = String::new();

    while let Some(chunk) = response.chunk().await? {
        let chunk_str = String::from_utf8_lossy(&chunk);
        buffer.push_str(&chunk_str);

        while let Some(event_end) = buffer.find("\n\n") {
            let event = buffer[..event_end].to_string();
            buffer = buffer[event_end + 2..].to_string();

            for line in event.split('\n') {
                if line.starts_with("data:") {
                    let data = &line["data:".len()..];

                    if data.trim() == "[DONE]" {
                        return Ok(());
                    }

                    let chunk: Chunk = match serde_json::from_str::<Chunk>(data) {
                        Ok(c) => c,
                        Err(e) => {
                            println!("data: {}\n", data);
                            println!("Error parsing chunk: {}", e);
                            return Ok(());
                        }
                    };
                    println!("{:#?}", chunk)
                }
            }
        }
    }

    Ok(())
}
