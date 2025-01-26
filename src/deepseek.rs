use serde::{ser::SerializeMap, Serializer};
use serde::{Deserialize, Serialize};
use thiserror::Error;

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

/// Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.
///
/// Number between -2.0 and 2.0.
/// # Examples
/// ```
/// # use crate::FrequencyPenalty;
/// let penalty = FrequencyPenalty::new(1.5).unwrap();
/// let too_high = FrequencyPenalty::new(3.0); // Returns Err(FrequencyPenaltyError::TooHigh)
/// let too_low = FrequencyPenalty::new(-3.0);  // Returns Err(FrequencyPenaltyError::TooLow)
/// ```
#[derive(Debug, Clone, Copy)]
pub struct FrequencyPenalty(f32);

#[derive(Debug)]
pub enum FrequencyPenaltyError {
    TooHigh,
    TooLow,
}

impl FrequencyPenalty {
    const MIN: f32 = -2.0;
    const MAX: f32 = 2.0;

    /// Creates a new FrequencyPenalty.
    ///
    /// # Errors
    /// Returns `FrequencyPenaltyError::TooLow` if value is less than -2.0
    /// Returns `FrequencyPenaltyError::TooHigh` if value is greater than 2.0
    const fn new(value: f32) -> Result<Self, FrequencyPenaltyError> {
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

impl TryFrom<f32> for FrequencyPenalty {
    type Error = FrequencyPenaltyError;

    /// Attempts to create FrequencyPenalty from f32 value
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

// TODO: response_format
// When using JSON Output, you must also instruct the model to produce JSON yourself via a system or user message.
// So we we will check the messages Vec if this is included? or is this to intrusive?

// TODO: stream
// SSE terminated by data:[DONE]

/// Higher values will make the output more random, while lower values will make it more focused and deterministic.
///
/// Number between 0.0 and 2.0.
/// ! Alter this or `top_p` but not both !
///
/// # Examples
/// ```
/// # use crate::Temperature;
/// let temp = Temperature::new(0.7).unwrap();
/// let too_high = Temperature::new(3.0); // Returns Err(TemperatureError::TooHigh)
/// let too_low = Temperature::new(-1.0);  // Returns Err(TemperatureError::TooLow)
///
/// // Use preset temperatures for different tasks
/// let coding = Temperature::CODING;      // 0.0 for deterministic code output
/// let poetry = Temperature::POETRY;      // 1.5 for creative poetry
/// ```
#[derive(Debug, Clone, Copy)]
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

impl TryFrom<f32> for Temperature {
    type Error = TemperatureError;

    fn try_from(temperature: f32) -> Result<Self, Self::Error> {
        Self::new(temperature)
    }
}

#[derive(Debug, Clone, Copy)]
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

impl TryFrom<f32> for TopP {
    type Error = TopPError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[derive(Serialize)]
pub enum ToolChoice {
    Mode(ToolChoiceMode),
    Tool(ToolCall),
}

impl ToolChoice {
    pub fn new<T: Into<Self>>(arg: T) -> Self {
        arg.into()
    }
}

impl From<ToolChoiceMode> for ToolChoice {
    fn from(mode: ToolChoiceMode) -> Self {
        ToolChoice::Mode(mode)
    }
}

impl From<String> for ToolChoice {
    fn from(name: String) -> Self {
        ToolChoice::Tool(ToolCall {
            type_: "function",
            function: FunctionName { name },
        })
    }
}

#[derive(Debug, Serialize)]
pub struct ToolCall {
    #[serde(rename = "type")]
    type_: &'static str,
    function: FunctionName,
}

#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct FunctionName {
    pub name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoiceMode {
    None,
    Auto,
    Required,
}

// TODO: logprobs

/// Number of most likely tokens to return at each token position, each with an associated log probability.
///
/// ! `logprobs` must be `true` to use this parameter !
///
/// Number between 0 and 20.
///
/// # Examples
/// ```
/// # use crate::TopLogProbs;
/// let probs = TopLogProbs::new(5).unwrap();
/// let too_high = TopLogProbs::new(21); // Returns Err(TopLogProbsError::TooHigh)
/// let too_low = TopLogProbs::new(-1);  // Returns Err(TopLogProbsError::TooLow)
/// ```
#[derive(Debug, Clone, Copy)]
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
