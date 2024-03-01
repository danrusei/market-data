mod client;
pub use client::{MarketClient, MarketData, Site};

mod publishers;
mod rest_call;

pub mod errors;
pub use errors::MarketError;
