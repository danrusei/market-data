pub(crate) mod iexcloud;

use crate::{client::MarketData, errors::MarketResult};

pub(crate) trait Publisher {
    fn create_endpoint(&mut self) -> MarketResult<()>;
    fn get_data(&mut self) -> MarketResult<()>;
    fn transform_data(&self) -> Option<MarketData>;
}
