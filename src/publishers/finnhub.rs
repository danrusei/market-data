//! Fetch time series stock data from [Finnhub](https://finnhub.io/docs/api), implements Publisher trait

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    client::{Interval, MarketSeries, Series},
    errors::{MarketError, MarketResult},
    publishers::Publisher,
};

const BASE_URL: &str = "https://finnhub.io/api/v1/";

/// Fetch time series stock data from [Finnhub](https://finnhub.io/docs/api), implements Publisher trait
#[derive(Debug)]
pub struct Finnhub {
    token: String,
}

#[derive(Debug, Clone)]
pub enum FinnhubRequest {
    Candle {
        symbol: String,
        resolution: String,
        from: i64,
        to: i64,
    },
    Quote {
        symbol: String,
    },
}

impl Finnhub {
    pub fn new(token: impl Into<String>) -> Self {
        Finnhub {
            token: token.into(),
        }
    }

    /// Request for daily series
    pub fn daily_series(&self, symbol: impl Into<String>, from: i64, to: i64) -> FinnhubRequest {
        FinnhubRequest::Candle {
            symbol: symbol.into(),
            resolution: "D".to_string(),
            from,
            to,
        }
    }

    /// Request for weekly series
    pub fn weekly_series(&self, symbol: impl Into<String>, from: i64, to: i64) -> FinnhubRequest {
        FinnhubRequest::Candle {
            symbol: symbol.into(),
            resolution: "W".to_string(),
            from,
            to,
        }
    }

    /// Request for monthly series
    pub fn monthly_series(&self, symbol: impl Into<String>, from: i64, to: i64) -> FinnhubRequest {
        FinnhubRequest::Candle {
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
        Ok(FinnhubRequest::Candle {
            symbol: symbol.into(),
            resolution,
            from,
            to,
        })
    }

    /// Request for real-time quote (returns a single bar)
    pub fn quote(&self, symbol: impl Into<String>) -> FinnhubRequest {
        FinnhubRequest::Quote {
            symbol: symbol.into(),
        }
    }
}

impl Publisher for Finnhub {
    type Request = FinnhubRequest;

    fn create_endpoint(&self, request: &Self::Request) -> MarketResult<Url> {
        let base_url = Url::parse(BASE_URL)?;
        match request {
            FinnhubRequest::Candle {
                symbol,
                resolution,
                from,
                to,
            } => {
                let mut url = base_url.join("stock/candle")?;
                url.query_pairs_mut()
                    .append_pair("symbol", symbol)
                    .append_pair("resolution", resolution)
                    .append_pair("from", &from.to_string())
                    .append_pair("to", &to.to_string())
                    .append_pair("token", &self.token);
                Ok(url)
            }
            FinnhubRequest::Quote { symbol } => {
                let mut url = base_url.join("quote")?;
                url.query_pairs_mut()
                    .append_pair("symbol", symbol)
                    .append_pair("token", &self.token);
                Ok(url)
            }
        }
    }

    fn transform_data(&self, data: String, request: &Self::Request) -> MarketResult<MarketSeries> {
        match request {
            FinnhubRequest::Candle {
                symbol, resolution, ..
            } => {
                let candles: FinnhubCandles = serde_json::from_str(&data)?;

                let status = match candles.status {
                    Some(ref s) => s.as_str(),
                    None => {
                        if let Some(ref err) = candles.error {
                            return Err(MarketError::DownloadedData(format!(
                                "Finnhub error: {}",
                                err
                            )));
                        }
                        return Err(MarketError::DownloadedData(format!(
                            "Finnhub response missing status. Response: {}",
                            data
                        )));
                    }
                };

                if status != "ok" {
                    return Err(MarketError::DownloadedData(format!(
                        "Error returned from Finnhub: {}",
                        candles.error.unwrap_or_else(|| status.to_string())
                    )));
                }

                let t = candles
                    .t
                    .ok_or_else(|| MarketError::DownloadedData("Missing timestamps".to_string()))?;
                let o = candles.o.ok_or_else(|| {
                    MarketError::DownloadedData("Missing open prices".to_string())
                })?;
                let h = candles.h.ok_or_else(|| {
                    MarketError::DownloadedData("Missing high prices".to_string())
                })?;
                let l = candles
                    .l
                    .ok_or_else(|| MarketError::DownloadedData("Missing low prices".to_string()))?;
                let c = candles.c.ok_or_else(|| {
                    MarketError::DownloadedData("Missing close prices".to_string())
                })?;
                let v = candles
                    .v
                    .ok_or_else(|| MarketError::DownloadedData("Missing volumes".to_string()))?;

                let mut data_series: Vec<Series> = Vec::with_capacity(t.len());
                for i in 0..t.len() {
                    let datetime = DateTime::from_timestamp(t[i], 0).ok_or_else(|| {
                        MarketError::ParsingError(format!("Unable to parse timestamp: {}", t[i]))
                    })?;

                    data_series.push(Series {
                        datetime: datetime.naive_utc(),
                        open: o[i],
                        close: c[i],
                        high: h[i],
                        low: l[i],
                        volume: v[i] as f64,
                    });
                }

                data_series.sort_by_key(|item| item.datetime);

                Ok(MarketSeries {
                    symbol: symbol.clone(),
                    interval: match resolution.as_str() {
                        "D" => Interval::Daily,
                        "W" => Interval::Weekly,
                        "M" => Interval::Monthly,
                        _ => Interval::Daily,
                    },
                    data: data_series,
                })
            }
            FinnhubRequest::Quote { symbol } => {
                let quote: FinnhubQuote = serde_json::from_str(&data)?;

                // If 't' is 0, it often means the symbol was not found
                if quote.t == 0 {
                    return Err(MarketError::DownloadedData(format!(
                        "Finnhub quote returned no data for symbol: {}",
                        symbol
                    )));
                }

                let datetime = DateTime::from_timestamp(quote.t, 0).ok_or_else(|| {
                    MarketError::ParsingError(format!("Unable to parse timestamp: {}", quote.t))
                })?;

                let series = Series {
                    datetime: datetime.naive_utc(),
                    open: quote.o,
                    close: quote.c,
                    high: quote.h,
                    low: quote.l,
                    volume: 0.0, // Quote doesn't return volume
                };

                Ok(MarketSeries {
                    symbol: symbol.clone(),
                    interval: Interval::Daily,
                    data: vec![series],
                })
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct FinnhubCandles {
    #[serde(rename = "c")]
    c: Option<Vec<f32>>,
    #[serde(rename = "h")]
    h: Option<Vec<f32>>,
    #[serde(rename = "l")]
    l: Option<Vec<f32>>,
    #[serde(rename = "o")]
    o: Option<Vec<f32>>,
    #[serde(rename = "s")]
    status: Option<String>,
    #[serde(rename = "t")]
    t: Option<Vec<i64>>,
    #[serde(rename = "v")]
    v: Option<Vec<u64>>,
    #[serde(rename = "error")]
    error: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct FinnhubQuote {
    #[serde(rename = "c")]
    c: f32,
    #[serde(rename = "h")]
    h: f32,
    #[serde(rename = "l")]
    l: f32,
    #[serde(rename = "o")]
    o: f32,
    #[serde(rename = "pc")]
    pc: f32,
    #[serde(rename = "t")]
    t: i64,
}
