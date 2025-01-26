#![allow(dead_code)]

// async fn example() {
//     let pool = deepseek::Pool::new("key", deepseek::Config::default());
//     // let pool = DeepseekConfig {
//     //     max_connections: 3,
//     //     client_builder: reqwest::ClientBuilder,
//     //     ..Default::default()
//     // }.connect("key").await;
//     //
//     // DeepseekConfig {
//     //     max_connections: 3,
//     //     client_builder: reqwest::ClientBuilder,
//     //     base_url: ""
//     //     ..Default::default()
//     // }

//     // let messages = DeepseekMessages::
//     //
//     let temp = deepseek::Temperature::try_from(1.0).unwrap();

//     // let temp = deepseek::Temperature::from(TemperaturePreset::Coding);
//     let temp = deespeek::Temperature::coding();
//     let temp = deepseek::Temperature::CODING;
// }

use crate::deepseek::{self, ToolChoice};

#[derive(Debug, Clone, Copy)]
struct Temperature(f32);

#[derive(Debug)]
enum TemperatureError {
    TooHigh,
    TooLow,
}

impl Temperature {
    const MIN: f32 = 0.0;
    const MAX: f32 = 1.5;

    const CODING: Self = Self(Self::MIN);
    const POETRY: Self = Self(Self::MAX);

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

fn test() {
    let temp = Temperature::new(1.0);
    let temp3 = Temperature::CODING;

    let config = deepseek::Config::default();
    let config2 = deepseek::Config {
        base_url: "https://openrouter.ai/api/v1",
        ..Default::default()
    };

    let choice = ToolChoice::Auto;
    let test = ToolChoice::Function("test".to_string());
}

// API
// struct Chat;
// struct Messages;
// strust Toolset;
// struct Role;
// struct Response;

// rgi::deepseek
