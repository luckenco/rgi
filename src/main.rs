use std::env;

use rgi::deepseek::{
    self,
    request::{Chat, Message, Temperature},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let client = deepseek::Client::new(
        &env::var("DEEPSEEK_KEY").expect("Missing DEEPSEEK_KEY"),
        deepseek::Config::default(),
    );

    let messages = vec![
        Message::System {
            content: String::from("You are a helpful assistant."),
            name: None,
        },
        Message::User {
            content: String::from("What's your favorite kind of synthetic data?"),
            name: None,
        },
    ];

    // Build the request
    let chat = Chat {
        messages,
        model: String::from("deepseek-chat"),
        frequency_penalty: None,
        max_tokens: None,
        presence_penalty: None,
        response_format: None,
        stop: None,
        stream: Some(false),
        stream_options: None,
        temperature: Some(Temperature::CONVERSATION),
        top_p: None,
        tools: None,
        tool_choice: None,
        logprobs: None,
        top_logprobs: None,
    };

    let response = client.complete(chat).await?;

    println!("{:#?}", response);

    Ok(())
}
