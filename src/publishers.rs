pub(crate) mod alphavantage;
pub(crate) mod finnhub;
pub(crate) mod massive;
pub(crate) mod twelvedata;
pub(crate) mod yahoo_finance;

use crate::{client::MarketSeries, errors::MarketResult};
use url::Url;

/// This trait has to be implemented by all the added sites
pub trait Publisher {
    type Request;
    fn create_endpoint(&self, request: &Self::Request) -> MarketResult<Url>;
    fn transform_data(&self, data: String, request: &Self::Request) -> MarketResult<MarketSeries>;
}
