//! Market-Data
//!
//! Market-Data - fetch & enhance historical time-series stock market data
//!
//! There are 2 components:
//!
//!   ==> Time-Series Download - historical Stock market time series download from supported Publishers
//!
//!   ==> Data transformation -  parse the downloaded data and enhance with selected indicators
//!
//! Check the [Readme file](https://github.com/danrusei/market-data) and the [Examples folder](https://github.com/danrusei/market-data/tree/main/examples) for more information.

mod client;
pub use client::{Interval, MarketClient, MarketSeries, Series};

mod publishers;
pub use publishers::{
    alphavantage::{AlphaVantage, OutputSize},
    iexcloud::Iex,
    polygon_io::Polygon,
    twelvedata::Twelvedata,
    yahoo_finance::{YahooFin, YahooRange},
};

mod indicators;
pub use indicators::{EnhancedMarketSeries, Indicators};

pub mod errors;
pub use errors::MarketError;

mod rest_call;
