use std::env;

use llm_playground::{
    client::LLMClient,
    providers::anthropic::{Anthropic, AnthropicRequestBuilder},
    Model,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let client = LLMClient::<Anthropic>::new(env::var("API_KEY").expect("Missing API_KEY"));

    let model = Model::<Anthropic>::new("claude-3-sonnet-20240229", 1024);

    let request = model
        .build_request()
        .temperature(0.7)
        .message("user", "Tell a joke");

    let response = client.chat_completion(request).await?;
    println!("{}", response.content.first().unwrap().text);

    Ok(())
}
