use std::env;

use rgi::deepseek::{
    self,
    request::{Chat, MaxTokens, Message},
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
        model: String::from("deepseek-reasoner"),
        max_tokens: Some(MaxTokens::default()),
        stop: None,
        stream: Some(false),
        stream_options: None,
    };

    let response = client.complete(chat).await?;

    println!("{:#?}", response);

    Ok(())
}
