mod client;
pub use client::{MarketClient, MarketData, Series};

mod publishers;
pub use publishers::{alphavantage::AlphaVantage, iexcloud::Iex};

pub mod errors;
pub use errors::MarketError;

mod rest_call;
