use std::env;

use llm_playground::{
    client::LLMClient,
    providers::{
        anthropic::{Anthropic, AnthropicRequestBuilder},
        deepseek::{DeepSeekRequestBuilder, Deepseek, Role},
    },
    Model,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let claude_client =
        LLMClient::<Anthropic>::new(env::var("ANTHROPIC_KEY").expect("Missing API_KEY"));
    let deepseek_client =
        LLMClient::<Deepseek>::new(env::var("DEEPSEEK_KEY").expect("Missing API_KEY"));

    let claude_model = Model::<Anthropic>::new("claude-3-sonnet-20240229", 1024);
    let deepseek_model = Model::<Deepseek>::new("deepseek-chat", 4096);

    let deepseek_request = deepseek_model
        .build_request()
        .message(Role::User, "Please roast Claude AI in a humorous way");

    let deepseek_response = deepseek_client.chat_completion(deepseek_request).await?;
    println!(
        "DeepSeek roasts Claude:\n{}\n",
        deepseek_response.choices.first().unwrap().message.content
    );

    let claude_request = claude_model
        .build_request()
        .temperature(0.7)
        .message("user", "Please roast DeepSeek AI in a humorous way");

    let claude_response = claude_client.chat_completion(claude_request).await?;
    println!(
        "Claude roasts DeepSeek:\n{}",
        claude_response.content.first().unwrap().text
    );

    Ok(())
}
