//! Fetch time series stock data from [Yahoo Finance](https://finance.yahoo.com/)
///
/// The Yahoo Finance API is free to use for personal projects. However, commercial usage of the API requires a paid subscription.
/// This means that developers working on commercial projects will need to pay for a Yahoo Finance API subscription.
///
/// The Yahoo Finance API is updated once per day. This means that developers will need to use other data sources if they want real-time data.
///
/// Example:
/// validRanges: 1d, 5d, 1mo, 3mo , 6mo, 1y, 2y, 5y, 10y, ytd, max
/// https://query1.finance.yahoo.com/v8/finance/chart/AAPL?metrics=high&interval=1d&range=5d
///
use chrono::{DateTime, NaiveDate};
use serde::{Deserialize, Serialize};
use std::fmt;
use url::Url;

use crate::{
    client::{Interval, MarketSeries, Series},
    errors::{MarketError, MarketResult},
    publishers::Publisher,
    rest_call::Client,
};

const BASE_URL: &str = "https://query1.finance.yahoo.com/v8/finance/chart/";

/// Fetch time series stock data from [Yahoo Finance](https://finance.yahoo.com/), implements Publisher trait
#[derive(Debug, Default)]
pub struct YahooFin {
    requests: Vec<YahooRequest>,
    endpoints: Vec<url::Url>,
    data: Vec<YahooPrices>,
    interval: Vec<Interval>,
}

#[derive(Debug, Default)]
pub struct YahooRequest {
    symbol: String,
    // The time interval between two data points supported by yahoo finance:1m, 2m, 5m, 15m, 30m, 60m, 90m, 1h, 1d, 5d, 1wk, 1mo, 3mo
    // I'm mapping to lib Interval struct
    interval: String,
    // validRanges: 1d, 5d, 1mo, 3mo , 6mo, 1y, 2y, 5y, 10y, ytd, max
    range: YahooRange,
}

#[derive(Debug, Default)]
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
    /// create new instance of Twelvedata
    pub fn new() -> Self {
        YahooFin {
            ..Default::default()
        }
    }

    /// Request for intraday series
    /// supporting the following intervals: 1min, 5min, 15min, 30min, 1h for intraday
    pub fn intraday_series(
        &mut self,
        symbol: impl Into<String>,
        interval: Interval,
        range: YahooRange,
    ) -> MarketResult<()> {
        self.interval.push(interval.clone());
        let interval = match interval {
            Interval::Min1 => "1m".to_string(),
            Interval::Min5 => "5m".to_string(),
            Interval::Min15 => "15m".to_string(),
            Interval::Min30 => "30m".to_string(),
            Interval::Hour1 => "1h".to_string(),
            _ => Err(MarketError::UnsuportedInterval(format!(
                "{} interval is not supported by AlphaVantage",
                interval
            )))?,
        };
        self.requests.push(YahooRequest {
            symbol: symbol.into(),
            interval,
            range,
        });
        Ok(())
    }

    /// Request for daily series
    pub fn daily_series(&mut self, symbol: impl Into<String>, range: YahooRange) -> () {
        self.interval.push(Interval::Daily);
        self.requests.push(YahooRequest {
            symbol: symbol.into(),
            interval: "1d".to_string(),
            range,
        });
    }

    /// Request for weekly series
    pub fn weekly_series(&mut self, symbol: impl Into<String>, range: YahooRange) -> () {
        self.interval.push(Interval::Weekly);
        self.requests.push(YahooRequest {
            symbol: symbol.into(),
            interval: "1wk".to_string(),
            range,
        });
    }

    /// Request for monthly series
    pub fn monthly_series(&mut self, symbol: impl Into<String>, range: YahooRange) -> () {
        self.interval.push(Interval::Monthly);
        self.requests.push(YahooRequest {
            symbol: symbol.into(),
            interval: "1m".to_string(),
            range,
        });
    }
}

impl Publisher for YahooFin {
    fn create_endpoint(&mut self) -> MarketResult<()> {
        let base_url = Url::parse(BASE_URL)?;
        self.endpoints = self
            .requests
            .iter()
            .map(|request| {
                let constructed_url = base_url
                    .join(&format!(
                        "{}?metrics=high&interval={}&range={}",
                        request.symbol, request.interval, request.range,
                    ))
                    .unwrap();
                constructed_url
            })
            .collect();
        // self.requests have to be consumed once used for creating the endpoints
        self.requests.clear();
        Ok(())
    }

    #[cfg(feature = "use-sync")]
    fn get_data(&mut self) -> MarketResult<()> {
        let rest_client = Client::new();
        for endpoint in &self.endpoints {
            let response = rest_client.get_data(endpoint)?;
            let body = response.into_string()?;

            let prices: YahooPrices = serde_json::from_str(&body)?;
            self.data.push(prices);
        }
        // self.endpoints have to be consumed once the data was downloaded for requested URL
        self.endpoints.clear();

        Ok(())
    }

    #[cfg(feature = "use-async")]
    async fn get_data(&mut self) -> MarketResult<()> {
        let client = Client::new();
        for endpoint in &self.endpoints {
            let response = client.get_data(endpoint).await?;
            let body = response.text().await?;

            let prices: YahooPrices = serde_json::from_str(&body)?;
            self.data.push(prices);
        }

        // self.endpoints have to be consumed once the data was downloaded for requested URL
        self.endpoints.clear();

        Ok(())
    }

    fn to_writer(&self, writer: impl std::io::Write) -> MarketResult<()> {
        serde_json::to_writer(writer, &self.data).map_err(|err| {
            MarketError::ToWriter(format!("Unable to write to writer, got the error: {}", err))
        })?;
        Ok(())
    }

    fn transform_data(&mut self) -> Vec<MarketResult<MarketSeries>> {
        let mut result = Vec::new();
        for (i, data) in self.data.iter().enumerate() {
            let parsed_data = transform(data, self.interval[i].clone());
            for data in parsed_data.into_iter() {
                result.push(data)
            }
        }

        // self.data have to be consumed once the data is transformed to MarketSeries
        self.data.clear();
        result
    }
}

// Yahoo API Deserialization

#[derive(Debug, Serialize, Deserialize)]
struct YahooPrices {
    chart: Chart,
}

#[derive(Debug, Serialize, Deserialize)]
struct Chart {
    result: Vec<Result>,
    error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Result {
    meta: Meta,
    timestamp: Vec<i64>,
    indicators: Indicators,
}

#[derive(Debug, Serialize, Deserialize)]
struct Meta {
    currency: String,
    symbol: String,
    #[serde(rename = "exchangeName")]
    exchange_name: String,
    #[serde(rename = "instrumentType")]
    instrument_type: String,
    #[serde(rename = "firstTradeDate")]
    first_trade_date: i64,
    #[serde(rename = "regularMarketTime")]
    regular_market_time: i64,
    #[serde(rename = "hasPrePostMarketData")]
    has_pre_post_market_data: bool,
    gmtoffset: i64,
    timezone: String,
    #[serde(rename = "exchangeTimezoneName")]
    exchange_timezone_name: String,
    #[serde(rename = "regularMarketPrice")]
    regular_market_price: f64,
    #[serde(rename = "chartPreviousClose")]
    chart_previous_close: f64,
    #[serde(rename = "priceHint")]
    price_hint: i32,
    #[serde(rename = "currentTradingPeriod")]
    current_trading_period: CurrentTradingPeriod,
    #[serde(rename = "dataGranularity")]
    data_granularity: String,
    range: String,
    #[serde(rename = "validRanges")]
    valid_ranges: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CurrentTradingPeriod {
    pre: TradingPeriod,
    regular: TradingPeriod,
    post: TradingPeriod,
}

#[derive(Debug, Serialize, Deserialize)]
struct TradingPeriod {
    timezone: String,
    end: i64,
    start: i64,
    gmtoffset: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Indicators {
    quote: Vec<Quote>,
    adjclose: Option<Vec<AdjClose>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Quote {
    volume: Vec<i64>,
    close: Vec<f64>,
    low: Vec<f64>,
    open: Vec<f64>,
    high: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AdjClose {
    adjclose: Vec<f64>,
}

fn transform(data: &YahooPrices, interval: Interval) -> Vec<MarketResult<MarketSeries>> {
    let mut result = Vec::new();

    // validate the data, first check is status
    if let Some(error) = &data.chart.error {
        result.push(Err(MarketError::DownloadedData(format!(
            "The return data has some error: {}",
            error
        ))));
    }

    for data in data.chart.result.iter() {
        let mut data_series: Vec<Series> = Vec::new();
        let mut timestamps: Vec<NaiveDate> = Vec::new();

        for timestamp in data.timestamp.iter() {
            // Create a NaiveDateTime from the Unix timestamp
            let datetime = DateTime::from_timestamp(timestamp.clone(), 0).ok_or(
                MarketError::ParsingError(format!("Unable to parse the timestamp")),
            );

            match datetime {
                Ok(datetime) => {
                    // Extract the date part
                    let date = datetime.date_naive();
                    timestamps.push(date);
                }
                Err(err) => {
                    result.push(Err(err));
                    // TO FIX !!!, need to continue with outer loop
                    continue;
                }
            }
        }

        for series in data.indicators.quote.iter() {
            for j in 1..series.open.len() - 1 {
                let open: f32 = series.open[j] as f32;
                let close: f32 = series.close[j] as f32;
                let high: f32 = series.high[j] as f32;
                let low: f32 = series.low[j] as f32;
                let volume: f32 = series.volume[j] as f32;

                data_series.push(Series {
                    date: timestamps[j],
                    open,
                    close,
                    high,
                    low,
                    volume,
                })
            }

            // sort the series by date
            //data_series.sort_by_key(|item| item.date);
        }
        result.push(Ok(MarketSeries {
            symbol: data.meta.symbol.clone(),
            interval: interval.clone(),
            data: data_series,
        }))
    }

    result
}

// validRanges: 1d, 5d, 1mo, 3mo , 6mo, 1y, 2y, 5y, 10y, ytd, max
impl fmt::Display for YahooRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let range_str = match self {
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

        write!(f, "{}", range_str)
    }
}
