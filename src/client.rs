use crate::{errors::MarketResult, publishers::Publisher, MarketError};
use serde::{Deserialize, Serialize};
use std::fmt;

pub struct MarketClient<T: Publisher> {
    inner: T,
}

impl<T: Publisher> MarketClient<T> {
    pub fn new(site: T) -> Self {
        MarketClient { inner: site }
    }
    pub fn create_endpoint(&mut self) -> MarketResult<()> {
        self.inner.create_endpoint()?;
        Ok(())
    }
    pub fn get_data(&mut self) -> MarketResult<()> {
        self.inner.get_data()?;
        Ok(())
    }
    pub fn transform_data(&self) -> MarketResult<MarketData> {
        let data = self.inner.transform_data().map_err(|err| {
            MarketError::DownloadedData(format!("Unable to transform the data: {}", err))
        })?;

        Ok(data)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketData {
    pub symbol: String,
    pub data: Vec<Series>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Series {
    pub date: String,
    pub open: f32,
    pub close: f32,
    pub high: f32,
    pub low: f32,
    pub volume: f32,
}

impl fmt::Display for MarketData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MarketData: Symbol={}, Data=\n{}",
            self.symbol,
            self.data
                .iter()
                .map(|series| format!("  {}", series))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl fmt::Display for Series {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Series: Date: {}, Open: {}, Close: {}, High: {}, Low: {}, Volume: {}",
            self.date, self.open, self.close, self.high, self.low, self.volume
        )
    }
}
