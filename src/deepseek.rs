use thiserror::Error;

pub mod completion;
pub mod request;

#[derive(Debug, Clone)]
pub struct Model {
    pub base_url: String,
    pub model: String,
    pub key: String,
}

// TODO: look into ArcStr crate
#[derive(Debug, Clone)]
pub enum Turn {
    User(String),
    Assistant(String),
}

#[derive(Debug, Clone)]
pub struct Messages {
    pub system: Option<String>,
    pub turns: Vec<Turn>,
}

// #[derive(Debug, Clone)]
// pub struct Tool {}

#[derive(Error, Debug)]
pub enum CompletionError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

pub async fn complete(
    client: reqwest::Client,
    model: &Model,
    messages: &Messages,
    // tools: &[Tool],
) -> Result<completion::Object, CompletionError> {
    let request_url = format!("{}/chat/completions", model.base_url);

    let request = request::Chat {
        messages: message_vec(messages),
        model: model.model.clone(),
        max_tokens: None, // TODO: figure out how to config this
        stream: Some(false),
    };

    let object = client
        .post(request_url)
        .json(&request)
        .send()
        .await?
        .json::<completion::Object>()
        .await?;

    Ok(object)
}

fn message_vec(messages: &Messages) -> Vec<request::Message> {
    // TODO: remove this whole shim, do the serialisation directly on Messages struct.
    // wayyy too much cloning but doing this for dev speed as we have those structs already

    let system_prompt_iter =
        messages
            .system
            .iter()
            .cloned()
            .map(|content| request::Message::System {
                content,
                name: None,
            });

    let turns_iter = messages.turns.iter().cloned().map(|turn| match turn {
        Turn::User(content) => request::Message::User {
            content,
            name: None,
        },
        Turn::Assistant(content) => request::Message::Assistant {
            content,
            name: None,
        },
    });

    system_prompt_iter.chain(turns_iter).collect()
}

// pub async fn stream(
//     client: &Client,
//     request: request::Chat,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let request_url = format!("{}/api/v1/chat/completions", client.config.base_url);

//     let body = json!(request);
//     println!("Request: {}", body);

//     let mut response = client
//         .inner
//         .post(&request_url)
//         .header("accept", "text/event-stream")
//         .body(body.to_string())
//         .send()
//         .await?
//         .error_for_status()?;

//     let mut buffer = String::new();

//     while let Some(chunk) = response.chunk().await? {
//         let chunk_str = String::from_utf8_lossy(&chunk);
//         buffer.push_str(&chunk_str);

//         while let Some(event_end) = buffer.find("\n\n") {
//             let event = buffer[..event_end].to_string();
//             buffer = buffer[event_end + 2..].to_string();

//             for line in event.split('\n') {
//                 if line.starts_with("data:") {
//                     let data = &line["data:".len()..];

//                     if data.trim() == "[DONE]" {
//                         return Ok(());
//                     }

//                     let chunk = match serde_json::from_str::<Chunk>(data) {
//                         Ok(c) => c,
//                         Err(e) => {
//                             println!("data: {}\n", data);
//                             println!("Error parsing chunk: {}", e);
//                             return Ok(());
//                         }
//                     };
//                     println!("{:#?}", chunk)
//                 }
//             }
//         }
//     }

//     Ok(())
// }
