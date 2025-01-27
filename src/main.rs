use std::{collections::HashMap, env};

use rgi::deepseek::request::{
    Chat, FrequencyPenalty, FunctionParameters, Message, Parameter, ResponseFormat, Stop,
    StreamOptions, Temperature, Tool, ToolChoice, TopLogProbs, TopP,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    let request = Chat {
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
