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

