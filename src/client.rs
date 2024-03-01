use reqwest::Response;
use url::Url;

use crate::{errors::MarketResult, MarketError};

pub struct Client {
    host: Url,
    inner_client: reqwest::Client,
}

impl Client {
    pub(crate) fn new(host: Url) -> Self {
        Client {
            host: host,
            inner_client: reqwest::Client::new(),
        }
    }
    pub(crate) async fn get_data(&self) -> MarketResult<Response> {
        let client = &self.inner_client;

        // Make an asynchronous GET request
        let response = client.get(self.host.clone()).send().await?;

        // Check if the request was successful, else send to user as Error
        let status_code = response.status();
        if status_code.is_success() {
            Ok(response)
        } else {
            Err(MarketError::HttpError(status_code.to_string()))
        }
    }
}
