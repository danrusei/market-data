use crate::{
    errors::MarketResult,
    indicators::{EnhancedMarketSeries, EnhancedSeries},
    publishers::Publisher,
    MarketError,
};
use serde::{Deserialize, Serialize};
use std::fmt;

/// MarketClient holds the Publisher
pub struct MarketClient<T: Publisher> {
    inner: T,
}

impl<T: Publisher> MarketClient<T> {
    pub fn new(site: T) -> Self {
        MarketClient { inner: site }
    }
    /// Creates the final query URL for the selected Provider
    pub fn create_endpoint(mut self) -> MarketResult<Self> {
        self.inner.create_endpoint()?;
        Ok(self)
    }
    /// Download the data series in the Provider format
    pub fn get_data(mut self) -> MarketResult<Self> {
        self.inner.get_data()?;
        Ok(self)
    }
    /// Transform the downloaded Provider series into MarketSeries format
    pub fn transform_data(&self) -> MarketResult<MarketSeries> {
        let data = self.inner.transform_data().map_err(|err| {
            MarketError::DownloadedData(format!("Unable to transform the data: {}", err))
        })?;

        Ok(data)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketSeries {
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

impl MarketSeries {
    pub fn enhance_data(self) -> EnhancedMarketSeries {
        let enhanced_series: Vec<EnhancedSeries> = self
            .data
            .iter()
            .map(|item| EnhancedSeries {
                date: item.date.clone(),
                open: item.open,
                close: item.close,
                high: item.high,
                low: item.low,
                volume: item.volume,
                ..Default::default()
            })
            .collect();
        EnhancedMarketSeries {
            symbol: self.symbol,
            data: enhanced_series,
        }
    }
}

impl fmt::Display for MarketSeries {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MarketSeries: Symbol={}, Series=\n{}",
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
            "Date: {}, Open: {}, Close: {}, High: {}, Low: {}, Volume: {}",
            self.date, self.open, self.close, self.high, self.low, self.volume
        )
    }
}
