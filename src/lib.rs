mod client;
pub use client::{MarketClient, MarketSeries, Series};

mod publishers;
pub use publishers::{
    alphavantage::{AlphaVantage, OutputSize},
    iexcloud::Iex,
};

pub mod errors;
pub use errors::MarketError;

mod rest_call;
