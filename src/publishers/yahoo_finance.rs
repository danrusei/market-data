//! Fetch time series stock data from [Yahoo Finance](https://finance.yahoo.com/)

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::fmt;
use url::Url;

use crate::{
    client::{Interval, MarketSeries, Series},
    errors::{MarketError, MarketResult},
    publishers::Publisher,
};

const BASE_URL: &str = "https://query1.finance.yahoo.com/v8/finance/chart/";

/// Fetch time series stock data from [Yahoo Finance](https://finance.yahoo.com/), implements Publisher trait
#[derive(Debug, Default)]
pub struct YahooFin {}

#[derive(Debug, Clone)]
pub struct YahooRequest {
    symbol: String,
    interval: String,
    range: YahooRange,
    interval_enum: Interval,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum YahooRange {
    Day1,
    Day5,
    Month1,
    Month3,
    #[default]
    Month6,
    Year1,
    Year2,
    Year5,
    Year10,
    Ytd,
    Max,
}

impl YahooFin {
    /// create new instance of YahooFin
    pub fn new() -> Self {
        YahooFin {}
    }

    /// Request for intraday series
    pub fn intraday_series(
        &self,
        symbol: impl Into<String>,
        interval: Interval,
        range: YahooRange,
    ) -> MarketResult<YahooRequest> {
        let interval_str = match interval {
            Interval::Min1 => "1m".to_string(),
            Interval::Min5 => "5m".to_string(),
            Interval::Min15 => "15m".to_string(),
            Interval::Min30 => "30m".to_string(),
            Interval::Hour1 => "1h".to_string(),
            _ => {
                return Err(MarketError::UnsuportedInterval(format!(
                    "{} interval is not supported by Yahoo Finance intraday",
                    interval
                )))
            }
        };
        Ok(YahooRequest {
            symbol: symbol.into(),
            interval: interval_str,
            range,
            interval_enum: interval,
        })
    }

    /// Request for daily series
    pub fn daily_series(&self, symbol: impl Into<String>, range: YahooRange) -> YahooRequest {
        YahooRequest {
            symbol: symbol.into(),
            interval: "1d".to_string(),
            range,
            interval_enum: Interval::Daily,
        }
    }

    /// Request for weekly series
    pub fn weekly_series(&self, symbol: impl Into<String>, range: YahooRange) -> YahooRequest {
        YahooRequest {
            symbol: symbol.into(),
            interval: "1wk".to_string(),
            range,
            interval_enum: Interval::Weekly,
        }
    }

    /// Request for monthly series
    pub fn monthly_series(&self, symbol: impl Into<String>, range: YahooRange) -> YahooRequest {
        YahooRequest {
            symbol: symbol.into(),
            interval: "1m".to_string(),
            range,
            interval_enum: Interval::Monthly,
        }
    }
}

impl Publisher for YahooFin {
    type Request = YahooRequest;

    fn create_endpoint(&self, request: &Self::Request) -> MarketResult<Url> {
        let base_url = Url::parse(BASE_URL)?;
        let mut url = base_url.join(&request.symbol)?;
        {
            let mut pairs = url.query_pairs_mut();
            pairs.append_pair("metrics", "high");
            pairs.append_pair("interval", &request.interval);
            pairs.append_pair("range", &request.range.to_string());
        }
        Ok(url)
    }

    fn transform_data(&self, data: String, request: &Self::Request) -> MarketResult<MarketSeries> {
        let yahoo_prices: YahooPrices = serde_json::from_str(&data)?;

        if let Some(error) = &yahoo_prices.chart.error {
            return Err(MarketError::DownloadedData(format!(
                "Yahoo Finance error: {}",
                error
            )));
        }

        if yahoo_prices.chart.result.is_empty() {
            return Err(MarketError::DownloadedData(
                "Yahoo Finance returned empty result".to_string(),
            ));
        }

        let result = &yahoo_prices.chart.result[0];
        let mut data_series: Vec<Series> = Vec::new();

        for (i, timestamp) in result.timestamp.iter().enumerate() {
            let datetime = DateTime::from_timestamp(*timestamp, 0).ok_or_else(|| {
                MarketError::ParsingError("Unable to parse timestamp".to_string())
            })?;

            if result.indicators.quote.is_empty() {
                continue;
            }

            let quote = &result.indicators.quote[0];

            // Check if values exist for this timestamp
            let open = quote.open.get(i).and_then(|v| *v);
            let high = quote.high.get(i).and_then(|v| *v);
            let low = quote.low.get(i).and_then(|v| *v);
            let close = quote.close.get(i).and_then(|v| *v);
            let volume = quote.volume.get(i).and_then(|v| *v);

            if let (Some(o), Some(h), Some(l), Some(c), Some(v)) = (open, high, low, close, volume)
            {
                data_series.push(Series {
                    datetime: datetime.naive_utc(),
                    open: o as f32,
                    high: h as f32,
                    low: l as f32,
                    close: c as f32,
                    volume: v as f64,
                });
            }
        }

        Ok(MarketSeries {
            symbol: result.meta.symbol.clone(),
            interval: request.interval_enum.clone(),
            data: data_series,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct YahooPrices {
    chart: Chart,
}

#[derive(Debug, Deserialize, Serialize)]
struct Chart {
    result: Vec<YahooResult>,
    error: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
struct YahooResult {
    meta: Meta,
    timestamp: Vec<i64>,
    indicators: Indicators,
}

#[derive(Debug, Deserialize, Serialize)]
struct Meta {
    symbol: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Indicators {
    quote: Vec<Quote>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Quote {
    volume: Vec<Option<i64>>,
    close: Vec<Option<f64>>,
    low: Vec<Option<f64>>,
    open: Vec<Option<f64>>,
    high: Vec<Option<f64>>,
}

impl fmt::Display for YahooRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            YahooRange::Day1 => "1d",
            YahooRange::Day5 => "5d",
            YahooRange::Month1 => "1mo",
            YahooRange::Month3 => "3mo",
            YahooRange::Month6 => "6mo",
            YahooRange::Year1 => "1y",
            YahooRange::Year2 => "2y",
            YahooRange::Year5 => "5y",
            YahooRange::Year10 => "10y",
            YahooRange::Ytd => "ytd",
            YahooRange::Max => "max",
        };
        write!(f, "{}", s)
    }
}
