pub mod client;
pub mod providers;

use std::marker::PhantomData;

use providers::Provider;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Clone)]
pub struct Model<P: Provider> {
    pub model: String,
    pub max_tokens: u32,
    pub config: P::Config,
    _provider: PhantomData<P>,
}

impl<P: Provider> Model<P>
where
    P::Config: Default,
    P::Request: From<Model<P>>,
{
    pub fn new(model: impl Into<String>, max_tokens: u32) -> Self {
        Self {
            model: model.into(),
            max_tokens,
            config: P::Config::default(),
            _provider: PhantomData,
        }
    }

    pub fn build_request(self) -> P::Request {
        self.into()
    }
}
