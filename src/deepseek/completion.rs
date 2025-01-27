use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Object {
    pub id: String,
    pub choices: Vec<Choice>,
    pub created: u32,
    pub model: String,
    pub system_fingerprint: String,
    pub object: ResponseObject,
    pub usage: Usage,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub finish_reason: FinishReason,
    pub index: u32,
    pub message: ResponseMessage,
    // TODO: logprobs
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    Stop,
    Length,
    ContentFilter,
    ToolCalls,
    InsufficientSystemResource,
}

#[derive(Debug, Deserialize)]
pub struct ResponseMessage {
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>,
    // TODO: Implement tool calls
    // pub tool_calls: Vec<ToolCall>,
    pub role: Role,
}

// #[derive(Debug, Deserialize)]
// pub struct ToolCall {
//     pub id: String,
//     #[serde(rename = "type")]
//     pub type_: ToolType,
//     pub function: FunctionCall,
// }

// #[derive(Debug, Deserialize)]
// pub struct FunctionCall {
//     pub name: String,
//     // Potentially JSON
//     pub arguments: String,
// }

#[derive(Debug, Default, Deserialize)]
pub enum ResponseObject {
    #[default]
    #[serde(rename = "chat.completion")]
    ChatCompletion,
    #[serde(rename = "chat.completion.chunk")]
    ChatCompletionChunk,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    #[default]
    Assistant,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolType {
    #[default]
    Function,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub completion_tokens: u32,
    pub prompt_tokens: u32,
    pub prompt_cache_hit_tokens: u32,
    pub prompt_cache_miss_tokens: u32,
    pub total_tokens: u32,
    // This field is mentioned in the docs but absent in the actual API response.
    // Instead of pub_token_details there is prompt_tokens_details
    // pub completion_token_details: CompletionTokenDetails,
}

// #[derive(Debug, Deserialize)]
// pub struct CompletionTokenDetails {
//     pub reasoning_tokens: u32,
// }

#[derive(Debug, Deserialize)]
pub struct Chunk {
    pub id: String,
    pub choices: Vec<Choice>,
    pub created: u32,
    pub model: String,
    pub system_fingerprint: String,
    pub object: ResponseObject,
    pub usage: Usage,
}

#[derive(Debug, Deserialize)]
pub struct StreamChoice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<FinishReason>,
    pub index: u32,
}

#[derive(Debug, Deserialize)]
pub struct Delta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
}
