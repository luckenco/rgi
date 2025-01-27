use model::REASONER;

pub mod completions;
pub mod request;

pub mod model {
    pub const CHAT: &'static str = "deepseek-chat";
    pub const REASONER: &'static str = "deepseek-reasoner";
}

pub struct Config {
    pub base_url: &'static str,
    pub model: &'static str,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_url: "https://api.deepseek.com",
            model: REASONER,
        }
    }
}

// TODO: response_format
// When using JSON Output, you must also instruct the model to produce JSON yourself via a system or user message.
// So we we will check the messages Vec if this is included? or is this to intrusive?

// TODO: stream
// SSE terminated by data:[DONE]

// TODO: tools
// Introduce function calling

// TODO: tool_choice
// Use serde annotations on the enum or impl From<_> ?

// TODO: logprobs
