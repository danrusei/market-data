//! Market-Data client implementation

use crate::{errors::MarketResult, indicators::EnhancedMarketSeries, publishers::Publisher};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt;

/// MarketClient holds the Publisher
pub struct MarketClient<T: Publisher> {
    pub site: T,
}

impl<T: Publisher> MarketClient<T> {
    pub fn new(site: T) -> Self {
        MarketClient { site }
    }

    /// Creates the final query URL for the selected Provider
    pub fn create_endpoint(mut self) -> MarketResult<Self> {
        self.site.create_endpoint()?;
        Ok(self)
    }

    /// Download the data series in the Provider format
    #[cfg(feature = "use-async")]
    pub async fn get_data(mut self) -> MarketResult<Self> {
        self.site.get_data().await?;
        Ok(self)
    }

    /// Download the data series in the Provider format
    #[cfg(feature = "use-sync")]
    pub fn get_data(mut self) -> MarketResult<Self> {
        self.site.get_data()?;
        Ok(self)
    }

    /// Write the downloaded data to anything that implements std::io::Write , like File, TcpStream, Stdout, etc
    pub fn to_writer(&self, writer: impl std::io::Write) -> MarketResult<()> {
        self.site.to_writer(writer)?;
        Ok(())
    }

    /// Transform the downloaded Provider series into MarketSeries format
    pub fn transform_data(&mut self) -> Vec<MarketResult<MarketSeries>> {
        self.site.transform_data()
    }
}

/// Holds the parsed data from Publishers
#[derive(Debug, Serialize, Deserialize)]
pub struct MarketSeries {
    pub symbol: String,
    pub interval: String,
    pub data: Vec<Series>,
}

/// Series part of the MarketSeries
#[derive(Debug, Serialize, Deserialize)]
pub struct Series {
    pub date: NaiveDate,
    pub open: f32,
    pub close: f32,
    pub high: f32,
    pub low: f32,
    pub volume: f32,
}

impl MarketSeries {
    pub fn enhance_data(self) -> EnhancedMarketSeries {
        EnhancedMarketSeries {
            series: self,
            asks: Vec::new(),
            indicators: Default::default(),
        }
    }
}

impl fmt::Display for MarketSeries {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MarketSeries: Symbol = {}, Interval = {}, Series =\n{}",
            self.symbol,
            self.interval,
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
