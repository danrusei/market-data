#[cfg(feature = "use-async")]
use reqwest::Response;
#[cfg(feature = "use-sync")]
use ureq::{Error as ureqError, Response};

use crate::{errors::MarketResult, MarketError};

pub struct Client {
    #[cfg(feature = "use-async")]
    inner_client: reqwest::Client,
    #[cfg(feature = "use-sync")]
    inner_client: ureq::Agent,
}

impl Client {
    pub(crate) fn new() -> Self {
        Client {
            #[cfg(feature = "use-async")]
            inner_client: reqwest::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .unwrap(),
            #[cfg(feature = "use-sync")]
            inner_client: ureq::AgentBuilder::new().build(),
        }
    }
    #[cfg(feature = "use-async")]
    pub(crate) async fn get_data(&self) -> MarketResult<Response> {
        let client = &self.inner_client;

        // Make an Asynchronous GET request
        let response = client.get(self.host.clone()).send().await?;

        // Check if the request was successful, else send to user as Error
        let status_code = response.status();
        if status_code.is_success() {
            Ok(response)
        } else {
            Err(MarketError::HttpError(status_code.to_string()))
        }
    }

    #[cfg(feature = "use-sync")]
    pub(crate) fn get_data(&self, endpoint: &url::Url) -> MarketResult<Response> {
        let client = &self.inner_client;

        // Make an Synchronous GET request
        match client.get(endpoint.as_str()).call() {
            Ok(response) => Ok(response),
            Err(ureqError::Status(code, response)) => {
                // the server returned an unexpected status code (such as 400, 500 etc)
                Err(MarketError::HttpError(format!(
                    "Got status {} with response: {:?}",
                    code,
                    response.into_string()
                )))
            }
            Err(err) => {
                // some kind of io/transport error
                Err(MarketError::HttpError(format!("got error: {}", err)))
            }
        }

        // Check if the request was successful, else send to user as Error
    }
}
