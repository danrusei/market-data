//! Fetch time series stock data from [Twelvedata](https://twelvedata.com/docs#time-series)

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    client::{Interval, MarketSeries, Series},
    errors::MarketResult,
    publishers::Publisher,
    MarketError,
};

const BASE_URL: &str = "https://api.twelvedata.com/time_series";

/// Fetch time series stock data from [Twelvedata](https://twelvedata.com/docs#time-series), implements Publisher trait
#[derive(Debug)]
pub struct Twelvedata {
    token: String,
}

#[derive(Debug)]
pub struct TDRequest {
    symbol: String,
    interval: String,
    output_size: u32,
}

impl Twelvedata {
    /// create new instance of Twelvedata
    pub fn new(token: impl Into<String>) -> Self {
        Twelvedata {
            token: token.into(),
        }
    }

    /// Request for intraday series
    pub fn intraday_series(
        &self,
        symbol: impl Into<String>,
        output_size: u32,
        interval: Interval,
    ) -> MarketResult<TDRequest> {
        let interval_str = match interval {
            Interval::Min1 => "1min".to_string(),
            Interval::Min5 => "5min".to_string(),
            Interval::Min15 => "15min".to_string(),
            Interval::Min30 => "30min".to_string(),
            Interval::Hour1 => "1h".to_string(),
            Interval::Hour2 => "2h".to_string(),
            Interval::Hour4 => "4h".to_string(),
            _ => {
                return Err(MarketError::UnsuportedInterval(format!(
                    "{} interval is not supported by Twelvedata",
                    interval
                )))
            }
        };
        Ok(TDRequest {
            symbol: symbol.into(),
            interval: interval_str,
            output_size,
        })
    }

    /// Request for daily series
    pub fn daily_series(&self, symbol: impl Into<String>, output_size: u32) -> TDRequest {
        TDRequest {
            symbol: symbol.into(),
            interval: "1day".to_string(),
            output_size,
        }
    }

    /// Request for weekly series
    pub fn weekly_series(&self, symbol: impl Into<String>, output_size: u32) -> TDRequest {
        TDRequest {
            symbol: symbol.into(),
            interval: "1week".to_string(),
            output_size,
        }
    }

    /// Request for monthly series
    pub fn monthly_series(&self, symbol: impl Into<String>, output_size: u32) -> TDRequest {
        TDRequest {
            symbol: symbol.into(),
            interval: "1month".to_string(),
            output_size,
        }
    }
}

impl Publisher for Twelvedata {
    type Request = TDRequest;

    fn create_endpoint(&self, request: &Self::Request) -> MarketResult<Url> {
        let base_url = Url::parse(BASE_URL)?;
        let mut url = base_url;
        url.query_pairs_mut()
            .append_pair("symbol", &request.symbol)
            .append_pair("interval", &request.interval)
            .append_pair("outputsize", &request.output_size.to_string())
            .append_pair("format", "json")
            .append_pair("apikey", &self.token);
        Ok(url)
    }

    fn transform_data(&self, data: String, _request: &Self::Request) -> MarketResult<MarketSeries> {
        let prices: TwelvedataPrices = serde_json::from_str(&data)?;

        if prices.status != "ok" {
            return Err(MarketError::DownloadedData(format!(
                "Downloaded data status is: {}",
                prices.status
            )));
        }

        let mut data_series: Vec<Series> = Vec::with_capacity(prices.time_series.len());

        for series in prices.time_series.iter() {
            let open: f32 = series.open.trim().parse().map_err(|e| {
                MarketError::ParsingError(format!("Unable to parse Open field: {}", e))
            })?;
            let close: f32 = series.close.trim().parse().map_err(|e| {
                MarketError::ParsingError(format!("Unable to parse Close field: {}", e))
            })?;
            let high: f32 = series.high.trim().parse().map_err(|e| {
                MarketError::ParsingError(format!("Unable to parse High field: {}", e))
            })?;
            let low: f32 = series.low.trim().parse().map_err(|e| {
                MarketError::ParsingError(format!("Unable to parse Low field: {}", e))
            })?;
            let volume: f64 = series.volume.trim().parse().map_err(|e| {
                MarketError::ParsingError(format!("Unable to parse Volume field: {}", e))
            })?;

            let datetime = if series.datetime.len() == 10 {
                NaiveDateTime::parse_from_str(&format!("{} 00:00:00", series.datetime), "%Y-%m-%d %H:%M:%S")
                    .map_err(|e| MarketError::ParsingError(format!("Unable to parse date: {}", e)))?
            } else {
                NaiveDateTime::parse_from_str(&series.datetime, "%Y-%m-%d %H:%M:%S").map_err(|e| {
                    MarketError::ParsingError(format!("Unable to parse datetime: {}", e))
                })?
            };

            data_series.push(Series {
                datetime,
                open,
                close,
                high,
                low,
                volume,
            })
        }

        // sort the series by date
        data_series.sort_by_key(|item| item.datetime);

        Ok(MarketSeries {
            symbol: prices.meta.symbol.clone(),
            interval: prices.meta.interval.clone().into(),
            data: data_series,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TwelvedataPrices {
    meta: MetaData,
    #[serde(rename(deserialize = "values"))]
    time_series: Vec<TimeSeriesData>,
    status: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct MetaData {
    symbol: String,
    interval: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TimeSeriesData {
    datetime: String,
    open: String,
    high: String,
    low: String,
    close: String,
    volume: String,
}
