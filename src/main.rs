use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    max_tokens: i32,
    messages: Vec<Message>,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    content: Vec<Content>,
}

#[derive(Debug, Deserialize)]
struct Content {
    text: String,
}

struct LLMClient {
    http_client: HttpClient,
    api_key: String,
    base_url: String,
}

impl LLMClient {
    fn new(api_key: String, base_url: String) -> Self {
        let http_client = HttpClient::new();
        Self {
            http_client,
            api_key,
            base_url,
        }
    }

    async fn chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, reqwest::Error> {
        let response = self
            .http_client
            .post(&self.base_url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let api_key = env::var("API_KEY").expect("Missing API_KEY");
    let base_url = env::var("BASE_URL").expect("Missing BASE_URL");

    let client = LLMClient::new(api_key, base_url);

    let request = ChatCompletionRequest {
        model: "claude-3-5-sonnet-20241022".to_string(),
        max_tokens: 1024,
        messages: vec![Message {
            role: "user".to_string(),
            content: "Tell me a joke about Rust".to_string(),
        }],
    };

    match client.chat_completion(request).await {
        Ok(response) => {
            if let Some(first_choice) = response.content.first() {
                println!("Response: {}", first_choice.text);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
