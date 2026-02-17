//! Fetch time series stock data from [Finnhub](https://finnhub.io/docs/api), implements Publisher trait

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    client::{Interval, MarketSeries, Series},
    errors::{MarketError, MarketResult},
    publishers::Publisher,
};

const BASE_URL: &str = "https://finnhub.io/api/v1/stock/candle";

/// Fetch time series stock data from [Finnhub](https://finnhub.io/docs/api), implements Publisher trait
#[derive(Debug)]
pub struct Finnhub {
    token: String,
}

#[derive(Debug)]
pub struct FinnhubRequest {
    symbol: String,
    resolution: String,
    from: i64,
    to: i64,
}

impl Finnhub {
    pub fn new(token: impl Into<String>) -> Self {
        Finnhub {
            token: token.into(),
        }
    }

    /// Request for daily series
    pub fn daily_series(&self, symbol: impl Into<String>, from: i64, to: i64) -> FinnhubRequest {
        FinnhubRequest {
            symbol: symbol.into(),
            resolution: "D".to_string(),
            from,
            to,
        }
    }

    /// Request for weekly series
    pub fn weekly_series(&self, symbol: impl Into<String>, from: i64, to: i64) -> FinnhubRequest {
        FinnhubRequest {
            symbol: symbol.into(),
            resolution: "W".to_string(),
            from,
            to,
        }
    }

    /// Request for monthly series
    pub fn monthly_series(&self, symbol: impl Into<String>, from: i64, to: i64) -> FinnhubRequest {
        FinnhubRequest {
            symbol: symbol.into(),
            resolution: "M".to_string(),
            from,
            to,
        }
    }

    /// Request for intraday series
    pub fn intraday_series(
        &self,
        symbol: impl Into<String>,
        from: i64,
        to: i64,
        interval: Interval,
    ) -> MarketResult<FinnhubRequest> {
        let resolution = match interval {
            Interval::Min1 => "1".to_string(),
            Interval::Min5 => "5".to_string(),
            Interval::Min15 => "15".to_string(),
            Interval::Min30 => "30".to_string(),
            Interval::Hour1 => "60".to_string(),
            _ => {
                return Err(MarketError::UnsuportedInterval(format!(
                    "{} interval is not supported by Finnhub",
                    interval
                )))
            }
        };
        Ok(FinnhubRequest {
            symbol: symbol.into(),
            resolution,
            from,
            to,
        })
    }
}

impl Publisher for Finnhub {
    type Request = FinnhubRequest;

    fn create_endpoint(&self, request: &Self::Request) -> MarketResult<Url> {
        let base_url = Url::parse(BASE_URL)?;
        let mut url = base_url;
        url.query_pairs_mut()
            .append_pair("symbol", &request.symbol)
            .append_pair("resolution", &request.resolution)
            .append_pair("from", &request.from.to_string())
            .append_pair("to", &request.to.to_string())
            .append_pair("token", &self.token);
        Ok(url)
    }

    fn transform_data(&self, data: String, request: &Self::Request) -> MarketResult<MarketSeries> {
        let candles: FinnhubCandles = serde_json::from_str(&data)?;

        if candles.status == "no_data" {
            return Err(MarketError::DownloadedData(
                "No data returned from Finnhub".to_string(),
            ));
        }

        let mut data_series: Vec<Series> = Vec::with_capacity(candles.t.len());
        for i in 0..candles.t.len() {
            let datetime = match DateTime::from_timestamp(candles.t[i], 0) {
                Some(dt) => dt.date_naive(),
                None => {
                    return Err(MarketError::ParsingError(format!(
                        "Unable to parse timestamp: {}",
                        candles.t[i]
                    )));
                }
            };

            data_series.push(Series {
                date: datetime,
                open: candles.o[i],
                close: candles.c[i],
                high: candles.h[i],
                low: candles.l[i],
                volume: candles.v[i] as f32,
            });
        }

        // sort the series by date
        data_series.sort_by_key(|item| item.date);

        Ok(MarketSeries {
            symbol: request.symbol.clone(),
            interval: match request.resolution.as_str() {
                "D" => Interval::Daily,
                "W" => Interval::Weekly,
                "M" => Interval::Monthly,
                _ => Interval::Daily, // or map more precisely
            },
            data: data_series,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct FinnhubCandles {
    #[serde(rename = "c")]
    c: Vec<f32>,
    #[serde(rename = "h")]
    h: Vec<f32>,
    #[serde(rename = "l")]
    l: Vec<f32>,
    #[serde(rename = "o")]
    o: Vec<f32>,
    #[serde(rename = "s")]
    status: String,
    #[serde(rename = "t")]
    t: Vec<i64>,
    #[serde(rename = "v")]
    v: Vec<u64>,
}
