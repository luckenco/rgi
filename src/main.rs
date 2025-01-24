use llm_playground::{AnthropicClient, Model};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let anthropic_client = AnthropicClient::new(std::env::var("API_KEY").expect("Missing API_KEY"));

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
