use std::env;

use rgi::deepseek::{
    self,
    request::{Chat, Message},
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
        ..Chat::default()
    };

    let response = client.complete(chat).await?;

    println!("{:#?}", response);

    Ok(())
}
