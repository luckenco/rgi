use std::env;

use rgi::deepseek::{
    self,
    request::{Chat, Message},
    stream,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let client = deepseek::Client::new(
        &env::var("DEEPSEEK_KEY").expect("Missing DEEPSEEK_KEY"),
        deepseek::Config::default(),
    );

    let messages = vec![Message::User {
        content: String::from("What's your favorite kind of synthetic data?"),
        name: None,
    }];

    // Build the request
    let chat = Chat {
        messages,
        stream: Some(true),
        ..Chat::default()
    };

    stream(&client, chat).await?;

    Ok(())
}
