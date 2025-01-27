# RGI

RGI is a Rust library that prioritizes an intuitive API design for LLM integration, enabling composable and intuitive AI interactions.

## Roadmap
_This is subject to change at any moment_


1. Implement seamless integration with `deepseek-chat`, crafting an intuitive API that feels natural
2. Optimize the deepseek provider for maximum performance and throughput
3. Expand our horizons by integrating Anthropic's powerful models
4. Create an elegant cross-provider abstraction layer:
   - Provider implementations will closely mirror their native APIs
   - A flexible wrapper type will enable seamless model interoperability
   - Opinionated but optional - use it when you need it
5. Build powerful agent construction tools to unlock advanced AI workflows

## Where we are going

```rust
let pool = deepseek::Pool::new("api_key", deepseek::Config::default());

let chat_request = Chat {
  messages: vec![
    Message::User {
      content: String::from("Why does TikTok love <thinking> outputs that much?"),
      name: None
    }
  ],
  model: deepseek::Model::Chat,
  ...deepseek::Chat::default()
};

let response = pool.dispatch(chat_request).await?;
```

Plugins can be applied through functional composition, enabling powerful enhancements. For example, you can boost Claude's completions by leveraging Deepseek's chain-of-thought reasoning capabilities:

```rust
let claude_request = Chat {
  messages: vec![
    Message::User {
      content: String::from("Why does TikTok love <thinking> outputs that much?"),
      name: None
    }
  ],
  model: anthropic::Model::ClaudeSonnet,
  ...anthropic::Chat::default()
};

let boosted_request = boost::ChainOfThough::new(claude_reqeuest);

let response = pool.dispatch(boosted_request);
```

### Goals

- Intuitive API design mirroring native provider interfaces
- Modular provider support through feature flags
- Universal request/response types for seamless provider interoperability
- Extensible plugin system for prompt enhancements (e.g. chain-of-thought reasoning)
- Robust compile-time validation of parameters
- Lightweight wrapper providing sensible defaults

## Where we are at

Example Usage:
```rust
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
```

### Todos

- [ ] Add `#[default]` derive macro to structs where appropriate to reduce boilerplate
- [ ] Implement comprehensive error handling with custom error types
- [ ] Support streaming responses for real-time output
- [ ] Add support for function/tool calling API
- [ ] Implement logprobs functionality for token probability analysis

**Brainstorming**

- Create a booster library for enhanced prompt performance:
  * Add chain-of-thought (CoT) reasoning with `.boost(CoT)` syntax
  * Leverage `deepseek-reasoner` model for efficient CoT generation
- Improve API design and safety:
  * Implement compile-time validation for requests
  * Add type-safe prompt templates
  * Ensure comprehensive request validation
- Enable seamless Python integration:
  * Design language-agnostic interfaces
  * Provide Python bindings/wrappers
  * Focus on minimal migration effort
- Explore Model Control Protocol (MCP) integration possibilities