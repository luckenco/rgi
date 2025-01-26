pub mod model {
    pub const CHAT: &'static str = "deepseek-chat";
    pub const REASONER: &'static str = "deepseek-reasoner";
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
struct FrequencyPenalty(f32);

#[derive(Debug)]
enum FrequencyPenaltyError {
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
struct Temperature(f32);

#[derive(Debug)]
enum TemperatureError {
    TooHigh,
    TooLow,
}

impl Temperature {
    const MIN: f32 = 0.0;
    const MAX: f32 = 2.0;

    const CODING: Self = Self(Self::MIN);
    const DATA: Self = Self(1.0);
    const CONVERSATION: Self = Self(1.3);
    const TRANSLATION: Self = Self(1.3);
    const POETRY: Self = Self(Self::MAX);

    /// Creates a new Temperature.
    ///
    /// # Errors
    /// Returns `TemperatureError::TooLow` if value is less than 0.0
    /// Returns `TemperatureError::TooHigh` if value is greater than 2.0
    const fn new(value: f32) -> Result<Self, TemperatureError> {
        match value {
            _ if value < Temperature::MIN => Err(TemperatureError::TooLow),
            _ if value > Temperature::MAX => Err(TemperatureError::TooHigh),
            _ => Ok(Self(value)),
        }
    }
}

impl TryFrom<f32> for Temperature {
    type Error = TemperatureError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
