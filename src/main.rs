use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client as HttpClient,
};
use serde::{Deserialize, Serialize};
use std::{env, marker::PhantomData};

struct Incomplete;
struct Complete;

struct AnthropicConfig;

trait LLMProvider {
    fn base_url(&self) -> &str;
    fn headers(&self) -> HeaderMap<HeaderValue>;
}

struct AnthropicClient {
    http_client: HttpClient,
    api_key: String,
    base_url: String,
}

impl AnthropicClient {
    fn new(api_key: impl Into<String>) -> Self {
        Self {
            http_client: HttpClient::new(),
            api_key: api_key.into(),
            base_url: "https://api.anthropic.com/v1/messages".to_string(),
        }
    }

    async fn chat_completion(
        &self,
        request: ModelRequest<Complete, AnthropicConfig>,
    ) -> Result<AnthropicResponse, Box<dyn std::error::Error>> {
        let response = self
            .http_client
            .post(&self.base_url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Clone)]
struct Model<S, P> {
    model: String,
    max_tokens: i32,
    system_prompt: Option<String>,
    _state: PhantomData<S>,
    _provider: PhantomData<P>,
}

#[derive(Serialize)]
struct ModelRequest<S, P> {
    model: String,
    max_tokens: i32,
    messages: Vec<Message>,

    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,

    #[serde(skip)]
    _state: PhantomData<S>,
    #[serde(skip)]
    _provider: PhantomData<P>,
}

impl Model<Incomplete, AnthropicConfig> {
    fn anthropic(model: impl Into<String>, max_tokens: i32) -> Self {
        Self {
            model: model.into(),
            max_tokens,
            system_prompt: None,
            _state: PhantomData,
            _provider: PhantomData,
        }
    }

    fn system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = Some(prompt.into());
        self
    }

    fn request(&self) -> ModelRequest<Incomplete, AnthropicConfig> {
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
    fn temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }

    fn message(mut self, role: impl Into<String>, content: impl Into<String>) -> Self {
        self.messages.push(Message {
            role: role.into(),
            content: content.into(),
        });
        self
    }

    fn build(self) -> Result<ModelRequest<Complete, AnthropicConfig>, &'static str> {
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

#[derive(Debug, Deserialize)]
struct TokenUsage {
    input_tokens: i64,
    output_tokens: i64,
}

#[derive(Debug, Deserialize)]
struct Content {
    text: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<Content>,
    usage: TokenUsage,
}

impl AnthropicResponse {
    fn get_text(&self) -> Option<&str> {
        self.content.first().map(|c| c.text.as_str())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let anthropic_client = AnthropicClient::new(env::var("API_KEY").expect("Missing API"));

    let sonnet = Model::anthropic("claude-3-sonnet-20240229", 1024)
        .system_prompt("You are a helpful AI assistant");

    let request = sonnet
        .request()
        .temperature(0.7)
        .message("user", "Tell me a joke")
        .build()?;

    let response = anthropic_client.chat_completion(request).await?;

    println!(
        "{}\n\nInput Tokens: {}\nOutput Tokens: {}",
        response.get_text().unwrap(),
        response.usage.input_tokens,
        response.usage.output_tokens
    );

    Ok(())
}
