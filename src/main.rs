use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone)]
enum ModelProvider {
    Anthropic,
    OpenAI,
    DeepSeek,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Message {
    role: String,
    content: String,
}

trait LLMRequest: Serialize {
    fn new(
        model: String,
        max_tokens: i32,
        messages: Vec<Message>,
        temperature: Option<f32>,
    ) -> Self;
}

trait LLMResponse {
    fn get_content(&self) -> Option<String>;
}

#[derive(Debug, Deserialize)]
struct TokenUsage {
    input_tokens: i64,
    output_tokens: i64,
}

#[derive(Debug, Deserialize)]
struct Content {
    text: String,
}

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: i32,
    messages: Vec<Message>,
    temperature: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<Content>,
    usage: TokenUsage,
}

impl LLMRequest for AnthropicRequest {
    fn new(
        model: String,
        max_tokens: i32,
        messages: Vec<Message>,
        temperature: Option<f32>,
    ) -> Self {
        Self {
            model,
            max_tokens,
            messages,
            temperature,
        }
    }
}

impl LLMResponse for AnthropicResponse {
    fn get_content(&self) -> Option<String> {
        self.content.first().map(|c| c.text.clone())
    }
}

struct LLMClient {
    http_client: HttpClient,
    api_key: String,
    base_url: String,
    provider: ModelProvider,
}

impl LLMClient {
    fn new(api_key: String, base_url: String, provider: ModelProvider) -> Self {
        let http_client = HttpClient::new();
        Self {
            http_client,
            api_key,
            base_url,
            provider,
        }
    }

    async fn chat_completion<Req: LLMRequest>(
        &self,
        request: Req,
    ) -> Result<String, Box<dyn std::error::Error>> {
        match self.provider {
            ModelProvider::Anthropic => {
                let response: AnthropicResponse = self
                    .http_client
                    .post(&self.base_url)
                    .header("x-api-key", &self.api_key)
                    .header("anthropic-version", "2023-06-01")
                    .json(&request)
                    .send()
                    .await?
                    .json()
                    .await?;

                Ok(response.get_content().unwrap_or_default())
            }
            ModelProvider::OpenAI => todo!("Implement OpenAI support"),
            ModelProvider::DeepSeek => todo!("Implement DeepSeek support"),
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let api_key = env::var("API_KEY").expect("Missing API_KEY");
    let base_url = env::var("BASE_URL").expect("Missing BASE_URL");

    let client = LLMClient::new(api_key, base_url, ModelProvider::Anthropic);

    let messages = vec![Message {
        role: "user".to_string(),
        content: "Tell me a joke about Rust".to_string(),
    }];

    let request = AnthropicRequest::new(
        "claude-3-5-sonnet-20241022".to_string(),
        1024,
        messages,
        Some(0.0),
    );

    match client.chat_completion(request).await {
        Ok(response) => println!("Response: {}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}
