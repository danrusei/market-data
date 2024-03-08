pub(crate) mod alphavantage;
// pub(crate) mod iexcloud;
pub(crate) mod twelvedata;

use crate::{client::MarketSeries, errors::MarketResult};

// This trait has to be implemented by all the added sites
pub trait Publisher {
    fn create_endpoint(&mut self) -> MarketResult<()>;
    #[cfg(feature = "use-async")]
    fn get_data(&mut self) -> impl std::future::Future<Output = MarketResult<()>> + Send;
    #[cfg(feature = "use-sync")]
    fn get_data(&mut self) -> MarketResult<()>;
    fn to_writer(&self, writer: impl std::io::Write) -> MarketResult<()>;
    fn transform_data(&mut self) -> Vec<MarketResult<MarketSeries>>;
}
