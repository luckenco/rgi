//! Slow and complicated serialization.
//!
//! A mix of both values being checked for being in range and still enabling
//! invalid states to be represented e.g. logprobs: false and top_logbrobs: Some(..).
//!
//! Fortunately, we aren't [redacted] enough to be unaware of this -> this package will be rewritten, once we are sure the API works how we want it to work.
use std::collections::HashMap;

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Clone, Serialize)]
pub struct Chat {
    pub messages: Vec<Message>,
    pub model: String,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub frequency_penalty: Option<FrequencyPenalty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<MaxTokens>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub presence_penalty: Option<PresencePenalty>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub response_format: Option<ResponseFormat>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub stop: Option<Stop>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub stream_options: Option<StreamOptions>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub temperature: Option<Temperature>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub top_p: Option<TopP>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub tools: Option<Vec<Tool>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub tool_choice: Option<ToolChoice>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub logprobs: Option<bool>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub top_logprobs: Option<TopLogProbs>,
}

impl Chat {
    pub fn default() -> Self {
        Self {
            messages: Vec::new(),
            model: String::from("deepseek/deepseek-r1-distill-llama-70b"),
            max_tokens: None,
            stream: Some(false),
        }
    }
}

/// The maximum length of the final response after the CoT output is completed,
/// defaulting to 4K, with a maximum of 8K. Note that the CoT output can reach up
/// to 32K tokens, and the parameter to control the CoT length (reasoning_effort)
/// will be available soon.
///
/// # Examples
/// ```
/// # use rgi::deepseek::primitives::{MaxTokens, MaxTokenError};
/// // Valid value creation
/// let tokens = MaxTokens::new(4000).unwrap();
/// assert_eq!(tokens.get(), 4000);
///
/// // Boundary violations
/// assert!(matches!(
///     MaxTokens::new(0),
///     Err(MaxTokenError::TooLow)
/// ));
///
/// assert!(matches!(
///     MaxTokens::new(9000),
///     Err(MaxTokenError::TooHigh)
/// ));
/// ```
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(transparent)]
pub struct MaxTokens(u16);

impl MaxTokens {
    pub const MIN: u16 = 1;
    pub const MAX: u16 = 8000;
    pub const DEFAULT: u16 = 4000;

    pub const fn new(value: u16) -> Result<Self, MaxTokenError> {
        match value {
            _ if value < MaxTokens::MIN => Err(MaxTokenError::TooLow),
            _ if value > MaxTokens::MAX => Err(MaxTokenError::TooHigh),
            _ => Ok(Self(value)),
        }
    }
}

impl Default for MaxTokens {
    fn default() -> Self {
        Self(Self::DEFAULT)
    }
}

#[derive(Error, Debug, Clone, Copy)]
pub enum MaxTokenError {
    #[error("max_tokens < {min} (MaxTokens::MIN)", min = MaxTokens::MIN)]
    TooLow,
    #[error("max_tokens > {max} (MaxTokens::MAX)", max = MaxTokens::MAX)]
    TooHigh,
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", content = "function", rename_all = "snake_case")]
pub enum Tool {
    Function {
        description: String,
        name: String,
        parameters: FunctionParameters,
    },
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FunctionParameters {
    Object {
        properties: HashMap<String, Parameter>,
        required: Vec<String>,
    },
}

#[derive(Debug, Serialize, Clone)]
pub struct Parameter {
    #[serde(rename = "type")]
    pub type_: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub enum ToolChoice {
    None,
    Auto,
    Required,
    Function(String),
}

impl serde::Serialize for ToolChoice {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ToolChoice::None => serializer.serialize_str("none"),
            ToolChoice::Auto => serializer.serialize_str("auto"),
            ToolChoice::Required => serializer.serialize_str("required"),
            ToolChoice::Function(name) => {
                #[derive(serde::Serialize)]
                struct ToolCall<'a> {
                    #[serde(rename = "type")]
                    type_: &'static str,
                    function: FunctionName<'a>,
                }

                #[derive(serde::Serialize)]
                struct FunctionName<'a> {
                    name: &'a str,
                }

                ToolCall {
                    type_: "function",
                    function: FunctionName { name },
                }
                .serialize(serializer)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum Message {
    System {
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    User {
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    Assistant {
        // Docs are stating this as 'nullable required'. For now content is required.
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    Tool {
        content: String,
        tool_call_id: String,
    },
}

/// Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.
///
/// Number between -2.0 and 2.0.
///
/// # Examples
/// ```
/// # use rgi::deepseek::primitives::{FrequencyPenalty, FrequencyPenaltyError};
/// // Valid value
/// let penalty = FrequencyPenalty::new(1.5).unwrap();
/// assert_eq!(penalty.get(), 1.5);
///
/// // Value too high
/// assert!(matches!(
///     FrequencyPenalty::new(3.0),
///     Err(FrequencyPenaltyError::TooHigh)
/// ));
///
/// // Value too low
/// assert!(matches!(
///     FrequencyPenalty::new(-3.0),
///     Err(FrequencyPenaltyError::TooLow)
/// ));
/// ```
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(transparent)]
pub struct FrequencyPenalty(f32);

#[derive(Error, Debug, Clone, Copy)]
pub enum FrequencyPenaltyError {
    #[error("frequency_penalty < {min} (FrequencyPenalty::MIN)", min = FrequencyPenalty::MIN)]
    TooLow,
    #[error("frequency_penalty > {max} (FrequencyPenalty::MAX)", max = FrequencyPenalty::MAX)]
    TooHigh,
}

impl FrequencyPenalty {
    pub const MIN: f32 = -2.0;
    pub const MAX: f32 = 2.0;
    pub const DEFAULT: f32 = 0.0;

    /// Creates a new FrequencyPenalty.
    ///
    /// # Errors
    /// Returns `FrequencyPenaltyError::TooLow` if value is less than -2.0
    /// Returns `FrequencyPenaltyError::TooHigh` if value is greater than 2.0
    pub const fn new(value: f32) -> Result<Self, FrequencyPenaltyError> {
        match value {
            _ if value < FrequencyPenalty::MIN => Err(FrequencyPenaltyError::TooLow),
            _ if value > FrequencyPenalty::MAX => Err(FrequencyPenaltyError::TooHigh),
            _ => Ok(Self(value)),
        }
    }

    /// Returns the inner f32 value
    pub const fn get(&self) -> f32 {
        self.0
    }
}

impl Default for FrequencyPenalty {
    fn default() -> Self {
        Self(Self::DEFAULT)
    }
}

impl TryFrom<f32> for FrequencyPenalty {
    type Error = FrequencyPenaltyError;

    /// Attempts to create FrequencyPenalty from f32 value
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.
///
/// Number between -2.0 and 2.0.
///
/// # Examples
/// ```
/// # use rgi::deepseek::primitives::{PresencePenalty, PresencePenaltyError};
/// // Valid value creation
/// let penalty = PresencePenalty::new(1.5).unwrap();
/// assert_eq!(penalty.get(), 1.5);
///
/// // Test boundary violations
/// assert!(matches!(
///     PresencePenalty::new(3.0),
///     Err(PresencePenaltyError::TooHigh)
/// ));
///
/// assert!(matches!(
///     PresencePenalty::new(-3.0),
///     Err(PresencePenaltyError::TooLow)
/// ));
/// ```
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(transparent)]
pub struct PresencePenalty(f32);

#[derive(Error, Debug, Clone, Copy)]
pub enum PresencePenaltyError {
    #[error("presence_penalty < {min} (PresencePenalty::MIN)", min = PresencePenalty::MIN)]
    TooLow,
    #[error("presence_penalty > {max} (PresencePenalty::MAX)", max = PresencePenalty::MAX)]
    TooHigh,
}

impl PresencePenalty {
    pub const MIN: f32 = -2.0;
    pub const MAX: f32 = 2.0;
    pub const DEFAULT: f32 = 0.0;

    /// Creates a new PresencePenalty.
    ///
    /// # Errors
    /// Returns `PresencePenaltyError::TooLow` if value is less than -2.0
    /// Returns `PresencePenaltyError::TooHigh` if value is greater than 2.0
    pub const fn new(value: f32) -> Result<Self, PresencePenaltyError> {
        match value {
            _ if value < PresencePenalty::MIN => Err(PresencePenaltyError::TooLow),
            _ if value > PresencePenalty::MAX => Err(PresencePenaltyError::TooHigh),
            _ => Ok(Self(value)),
        }
    }

    /// Returns the inner f32 value
    pub const fn get(&self) -> f32 {
        self.0
    }
}

impl Default for PresencePenalty {
    fn default() -> Self {
        Self(Self::DEFAULT)
    }
}

impl TryFrom<f32> for PresencePenalty {
    type Error = PresencePenaltyError;

    /// Attempts to create PresencePenalty from f32 value
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormat {
    #[default]
    Text,
    JsonObject,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(transparent)]
pub struct Stop(Vec<String>);

impl Stop {
    pub const MAX_LEN: usize = 16;

    pub fn new(stop: Vec<String>) -> Result<Self, StopError> {
        #[allow(clippy::unnecessary_lazy_evaluations)]
        (stop.len() <= Self::MAX_LEN)
            .then(|| Self(stop))
            .ok_or(StopError::TooManyStops)
    }

    pub fn get(&self) -> &[String] {
        &self.0
    }
}

#[derive(Error, Debug, Clone, Copy)]
pub enum StopError {
    #[error("stop.len() > {max} (Stop::MAX_LEN)", max = Stop::MAX_LEN)]
    TooManyStops,
}

#[derive(Debug, Clone, Serialize)]
pub struct StreamOptions {
    pub include_usage: Option<bool>,
}

/// Higher values will make the output more random, while lower values will make it more focused and deterministic.
///
/// Number between 0.0 and 2.0.
/// ! Alter this or `top_p` but not both !
///
/// # Examples
/// ```
/// # use rgi::deepseek::primitives::{Temperature, TemperatureError};
/// // Valid value creation
/// let temp = Temperature::new(0.7).unwrap();
/// assert_eq!(temp.get(), 0.7);
///
/// // Boundary violations
/// assert!(matches!(
///     Temperature::new(3.0),
///     Err(TemperatureError::TooHigh)
/// ));
///
/// assert!(matches!(
///     Temperature::new(-1.0),
///     Err(TemperatureError::TooLow)
/// ));
///
/// // Preset temperature checks
/// assert_eq!(Temperature::CODING.get(), 0.0);
/// assert_eq!(Temperature::DATA.get(), 1.0);
/// assert_eq!(Temperature::CONVERSATION.get(), 1.3);
/// assert_eq!(Temperature::TRANSLATION.get(), 1.3);
/// assert_eq!(Temperature::POETRY.get(), 2.0);
/// ```
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(transparent)]
pub struct Temperature(f32);

#[derive(Error, Debug, Clone, Copy)]
pub enum TemperatureError {
    #[error("temperature < {min} (Temperature::MIN)", min = Temperature::MIN)]
    TooLow,
    #[error("temperature > {max} (Temperature::MAX)", max = Temperature::MAX)]
    TooHigh,
}

impl Temperature {
    pub const MIN: f32 = 0.0;
    pub const MAX: f32 = 2.0;
    pub const DEFAULT: f32 = 1.0;

    pub const CODING: Self = Self(Self::MIN);
    pub const DATA: Self = Self(1.0);
    pub const CONVERSATION: Self = Self(1.3);
    pub const TRANSLATION: Self = Self(1.3);
    pub const POETRY: Self = Self(Self::MAX);

    /// Creates a new Temperature.
    ///
    /// # Errors
    /// Returns `TemperatureError::TooLow` if value is less than 0.0
    /// Returns `TemperatureError::TooHigh` if value is greater than 2.0
    pub const fn new(temperature: f32) -> Result<Self, TemperatureError> {
        match temperature {
            _ if temperature < Temperature::MIN => Err(TemperatureError::TooLow),
            _ if temperature > Temperature::MAX => Err(TemperatureError::TooHigh),
            _ => Ok(Self(temperature)),
        }
    }

    /// Returns the inner f32 value
    pub const fn get(&self) -> f32 {
        self.0
    }
}

impl Default for Temperature {
    fn default() -> Self {
        Self(Self::DEFAULT)
    }
}

impl TryFrom<f32> for Temperature {
    type Error = TemperatureError;

    fn try_from(temperature: f32) -> Result<Self, Self::Error> {
        Self::new(temperature)
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct TopP(f32);

#[derive(Error, Debug, Clone, Copy)]
pub enum TopPError {
    #[error("top_p < {min} (TopP::MIN)", min = TopP::MIN)]
    TooLow,
    #[error("top_p > {max} (TopP::MAX)", max = TopP::MAX)]
    TooHigh,
}

impl TopP {
    pub const MIN: f32 = 0.0;
    pub const MAX: f32 = 1.0;
    pub const DEFAULT: f32 = 1.0;

    /// Creates a new TopP.
    ///
    /// # Errors
    /// Returns `TopPError::TooLow` if value is less than 0.0
    /// Returns `TopPError::TooHigh` if value is greater than 1.0
    pub const fn new(top_p: f32) -> Result<Self, TopPError> {
        match top_p {
            _ if top_p < TopP::MIN => Err(TopPError::TooLow),
            _ if top_p > TopP::MAX => Err(TopPError::TooHigh),
            _ => Ok(Self(top_p)),
        }
    }

    /// Returns the inner f32 value
    pub const fn get(&self) -> f32 {
        self.0
    }
}

impl Default for TopP {
    fn default() -> Self {
        Self(Self::DEFAULT)
    }
}

impl TryFrom<f32> for TopP {
    type Error = TopPError;

    fn try_from(top_p: f32) -> Result<Self, Self::Error> {
        Self::new(top_p)
    }
}

/// Number of most likely tokens to return at each token position, each with an associated log probability.
///
/// ! `logprobs` must be `true` to use this parameter !
///
/// Number between 0 and 20.
///
/// # Examples
/// ```
/// # use rgi::deepseek::primitives::{TopLogProbs, TopLogProbsError};
/// // Valid value creation
/// let probs = TopLogProbs::new(5).unwrap();
/// assert_eq!(probs.get(), 5);
///
/// // Test boundary values
/// let min_probs = TopLogProbs::new(0).unwrap();
/// assert_eq!(min_probs.get(), 0);
/// let max_probs = TopLogProbs::new(20).unwrap();
/// assert_eq!(max_probs.get(), 20);
///
/// // Test error conditions
/// assert!(matches!(TopLogProbs::new(21), Err(TopLogProbsError::TooHigh)));
/// assert!(matches!(TopLogProbs::new(-1), Err(TopLogProbsError::TooLow)));
/// ```
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(transparent)]
pub struct TopLogProbs(i32);

#[derive(Error, Debug, Clone, Copy)]
pub enum TopLogProbsError {
    #[error("top_logprobs < {min} (TopLogProbs::MIN)", min = TopLogProbs::MIN)]
    TooLow,
    #[error("top_logprobs > {max} (TopLogProbs::MAX)", max = TopLogProbs::MAX)]
    TooHigh,
}

impl TopLogProbs {
    pub const MIN: i32 = 0;
    pub const MAX: i32 = 20;

    /// Creates a new TopLogProbs.
    ///
    /// # Errors
    /// Returns `TopLogProbsError::TooLow` if value is less than 0
    /// Returns `TopLogProbsError::TooHigh` if value is greater than 20
    pub const fn new(top_logprobs: i32) -> Result<Self, TopLogProbsError> {
        match top_logprobs {
            _ if top_logprobs < TopLogProbs::MIN => Err(TopLogProbsError::TooLow),
            _ if top_logprobs > TopLogProbs::MAX => Err(TopLogProbsError::TooHigh),
            _ => Ok(Self(top_logprobs)),
        }
    }

    /// Returns the inner i32 value
    pub const fn get(&self) -> i32 {
        self.0
    }
}

impl TryFrom<i32> for TopLogProbs {
    type Error = TopLogProbsError;

    fn try_from(top_logprobs: i32) -> Result<Self, Self::Error> {
        Self::new(top_logprobs)
    }
}

// TODO
pub struct FIMCompletion {}
