pub(crate) mod alphavantage;
pub(crate) mod iexcloud;
pub(crate) mod twelvedata;

use crate::{client::MarketSeries, errors::MarketResult};

pub trait Publisher {
    fn create_endpoint(&mut self) -> MarketResult<()>;
    fn get_data(&mut self) -> MarketResult<()>;
    fn to_writer(&self, writer: impl std::io::Write) -> MarketResult<()>;
    fn transform_data(&self) -> MarketResult<MarketSeries>;
}
