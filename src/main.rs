use std::{collections::HashMap, env};

use rgi::{
    client::LLMClient,
    deepseek::primitives::{
        FrequencyPenalty, FunctionParameters, Message, Parameter, Request, ResponseFormat, Stop,
        StreamOptions, Temperature, Tool, ToolChoice, TopLogProbs, TopP,
    },
    providers::{
        anthropic::{Anthropic, AnthropicRequestBuilder},
        deepseek::{DeepSeekRequestBuilder, Deepseek, Role},
    },
    Model,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // dotenv::dotenv().ok();

    // let claude_client =
    //     LLMClient::<Anthropic>::new(env::var("ANTHROPIC_KEY").expect("Missing API_KEY"));
    // let deepseek_client =
    //     LLMClient::<Deepseek>::new(env::var("DEEPSEEK_KEY").expect("Missing API_KEY"));

    // let claude_model = Model::<Anthropic>::new("claude-3-sonnet-20240229", 1024);
    // let deepseek_model = Model::<Deepseek>::new("deepseek-chat", 4096);

    // let deepseek_request = deepseek_model
    //     .build_request()
    //     .message(Role::User, "Please roast Claude AI in a humorous way");

    // let deepseek_response = deepseek_client.chat_completion(deepseek_request).await?;
    // println!(
    //     "DeepSeek roasts Claude:\n{}\n",
    //     deepseek_response.choices.first().unwrap().message.content
    // );

    // let claude_request = claude_model
    //     .build_request()
    //     .temperature(0.7)
    //     .message("user", "Please roast DeepSeek AI in a humorous way");

    // let claude_response = claude_client.chat_completion(claude_request).await?;
    // println!(
    //     "Claude roasts DeepSeek:\n{}",
    //     claude_response.content.first().unwrap().text
    // );

    let messages = vec![
        Message::System {
            content: String::from("You are a helpful assistant."),
            name: None,
        },
        Message::User {
            content: String::from("What's the weather like in Boston?"),
            name: None,
        },
    ];

    // Create a weather tool function
    let weather_tool = Tool::Function {
        description: String::from("Get the current weather in a location"),
        name: String::from("weather_get"),
        parameters: FunctionParameters::Object {
            properties: {
                let mut props = HashMap::new();
                props.insert(
                    String::from("location"),
                    Parameter {
                        type_: String::from("string"),
                        description: String::from("The city and state, e.g. San Francisco, CA"),
                    },
                );
                props.insert(
                    String::from("unit"),
                    Parameter {
                        type_: String::from("string"),
                        description: String::from("Temperature unit: 'celsius' or 'fahrenheit'"),
                    },
                );
                props
            },
            required: vec![String::from("location")],
        },
    };

    // Build the request
    let request = Request {
        messages,
        model: String::from("deepseek-7b"),
        frequency_penalty: Some(FrequencyPenalty::new(0.5).unwrap()),
        max_tokens: None,
        presence_penalty: None,
        response_format: Some(ResponseFormat::JsonObject),
        stop: Some(Stop::new(vec!["</think>".into()]).unwrap()),
        stream: Some(false),
        stream_options: Some(StreamOptions {
            include_usage: Some(true),
        }),
        temperature: Some(Temperature::CONVERSATION),
        top_p: Some(TopP::default()),
        tools: Some(vec![weather_tool]),
        tool_choice: Some(ToolChoice::Function("weather_get".to_string())),
        logprobs: Some(true),
        top_logprobs: Some(TopLogProbs::new(0).unwrap()),
    };

    // Serialize and print
    println!("{}", serde_json::to_string_pretty(&request).unwrap());

    Ok(())
}
