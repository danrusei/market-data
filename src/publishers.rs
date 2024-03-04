pub(crate) mod alphavantage;
pub(crate) mod iexcloud;

use crate::{client::MarketSeries, errors::MarketResult};

pub trait Publisher {
    fn create_endpoint(&mut self) -> MarketResult<()>;
    fn get_data(&mut self) -> MarketResult<()>;
    fn transform_data(&self) -> MarketResult<MarketSeries>;
}
