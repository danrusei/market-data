//! Market-Data client implementation

use crate::{errors::MarketResult, indicators::EnhancedMarketSeries, publishers::Publisher};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// MarketClient holds the Publisher and reqwest::Client
pub struct MarketClient<T: Publisher> {
    pub site: T,
    inner: reqwest::Client,
}

impl<T: Publisher> MarketClient<T> {
    pub fn new(site: T) -> Self {
        Self {
            site,
            inner: reqwest::Client::new(),
        }
    }

    /// Fetches the data for a single request
    pub async fn fetch(&self, request: T::Request) -> MarketResult<MarketSeries> {
        let url = self.site.create_endpoint(&request)?;
        let response = self.inner.get(url).send().await?;
        let body = response.text().await?;
        self.site.transform_data(body, &request)
    }
}

/// Holds the parsed data from Publishers
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarketSeries {
    /// holds symbol like: "GOOGL"
    pub symbol: String,
    /// inteval from intraday to monthly
    pub interval: Interval,
    /// the original series downloaded and parsed from publishers
    pub data: Vec<Series>,
}

/// Series part of the MarketSeries
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Series {
    /// the date of the stock price
    pub date: NaiveDate,
    /// the opening price of the stock for the selected interval
    pub open: f32,
    /// the closing price of the stock for the selected interval
    pub close: f32,
    /// the highest price of the stock for the selected interval
    pub high: f32,
    /// the lowest price of the stock for the selected interval
    pub low: f32,
    /// the number of shares traded in the selected interval
    pub volume: f32,
}

/// The time interval between two data points
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub enum Interval {
    /// 1 minute interval
    Min1,
    /// 5 minutes interval
    Min5,
    /// 15 minutes interval
    Min15,
    /// 30 minutes interval
    Min30,
    /// 1 hour interval
    Hour1,
    /// 2 hours interval
    Hour2,
    /// 4 hours interval
    Hour4,
    /// daily interval
    #[default]
    Daily,
    /// weekly interval
    Weekly,
    /// monthly interval
    Monthly,
}

impl MarketSeries {
    pub fn enhance_data(self) -> EnhancedMarketSeries {
        EnhancedMarketSeries {
            symbol: self.symbol,
            interval: self.interval,
            series: self.data,
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

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let interval_str = match self {
            Interval::Min1 => "1 minute",
            Interval::Min5 => "5 minutes",
            Interval::Min15 => "15 minutes",
            Interval::Min30 => "30 minutes",
            Interval::Hour1 => "1 hour",
            Interval::Hour2 => "2 hours",
            Interval::Hour4 => "4 hours",
            Interval::Daily => "Daily",
            Interval::Weekly => "Weekly",
            Interval::Monthly => "Monthly",
        };

        write!(f, "{}", interval_str)
    }
}

/// AlphaVantage response interval: 1min, 5min, 15min, 30min, 60min,
/// Twelvedata response interval: 1min, 5min, 15min, 30min, 1h, 2h, 4h, 1day, 1week, 1month
impl std::str::FromStr for Interval {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "1min" => Ok(Interval::Min1),
            "5min" => Ok(Interval::Min5),
            "15min" => Ok(Interval::Min15),
            "30min" => Ok(Interval::Min30),
            "60min" => Ok(Interval::Hour1),
            "1h" => Ok(Interval::Hour1),
            "2h" => Ok(Interval::Hour2),
            "4h" => Ok(Interval::Hour4),
            "1day" => Ok(Interval::Daily),
            "1week" => Ok(Interval::Weekly),
            "1month" => Ok(Interval::Monthly),
            _ => Err("Invalid interval string"),
        }
    }
}

impl From<String> for Interval {
    fn from(s: String) -> Self {
        Interval::from_str(&s).unwrap_or(Interval::Daily)
    }
}
