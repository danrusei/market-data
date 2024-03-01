mod client;
pub use client::{MarketClient, MarketData};

mod publishers;
pub use publishers::iexcloud::Iex;

pub mod errors;
pub use errors::MarketError;

mod rest_call;
