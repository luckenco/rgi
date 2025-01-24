use std::marker::PhantomData;

use reqwest::Client;

use crate::providers::Provider;

pub struct LLMClient<P: Provider> {
    http_client: Client,
    api_key: String,
    _phantom: PhantomData<P>,
}

impl<P: Provider> LLMClient<P> {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            http_client: Client::new(),
            api_key: api_key.into(),
            _phantom: PhantomData,
        }
    }

    pub async fn chat_completion(
        &self,
        request: P::Request,
    ) -> Result<P::Response, Box<dyn std::error::Error>> {
        let url = P::base_url();
        let headers = P::headers(&self.api_key);

        let response = self
            .http_client
            .post(url)
            .headers(headers)
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
}
