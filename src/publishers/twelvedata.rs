//! Fetch time series stock data from [Twelvedata](https://twelvedata.com/docs#time-series)
///
/// Example Daily requests:
/// https://api.twelvedata.com/time_series?symbol=AAPL&interval=1day&outputsize=50&apikey=your_api_key
/// outputsize parameter : Number of data points to retrieve, Supports values in the range from 1 to 5000, Default is 30
/// inteval : Supports: 1min, 5min, 15min, 30min, 45min, 1h, 2h, 4h, 1day, 1week, 1month
///
/// Example intraday requests:
/// https://api.twelvedata.com/time_series?symbol=AAPL&interval=15min&apikey=your_api_key
/// https://api.twelvedata.com/time_series?symbol=AAPL&interval=1h&apikey=your_api_key
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    client::{Interval, MarketSeries, Series},
    errors::MarketResult,
    publishers::Publisher,
    rest_call::Client,
    MarketError,
};

const BASE_URL: &str = "https://api.twelvedata.com/time_series";

/// Fetch time series stock data from [Twelvedata](https://twelvedata.com/docs#time-series), implements Publisher trait
#[derive(Debug, Default)]
pub struct Twelvedata {
    token: String,
    requests: Vec<TDRequest>,
    endpoints: Vec<url::Url>,
    data: Vec<TwelvedataPrices>,
}

#[derive(Debug, Default)]
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
            ..Default::default()
        }
    }

    /// Request for intraday series
    /// it supports only the following intervals: 1min, 5min, 15min, 30min, 45min, 1h, 2h, 4h
    pub fn intraday_series(
        &mut self,
        symbol: impl Into<String>,
        output_size: u32,
        interval: Interval,
    ) -> MarketResult<()> {
        let interval = match interval {
            Interval::Min1 => "1min".to_string(),
            Interval::Min5 => "5min".to_string(),
            Interval::Min15 => "15min".to_string(),
            Interval::Min30 => "30min".to_string(),
            Interval::Hour1 => "1h".to_string(),
            Interval::Hour1 => "2h".to_string(),
            Interval::Hour1 => "4h".to_string(),
            _ => Err(MarketError::UnsuportedInterval(format!(
                "{} interval is not supported by AlphaVantage",
                interval
            )))?,
        };
        self.requests.push(TDRequest {
            symbol: symbol.into(),
            interval,
            output_size,
        });
        Ok(())
    }

    /// Request for daily series
    pub fn daily_series(&mut self, symbol: impl Into<String>, output_size: u32) -> () {
        self.requests.push(TDRequest {
            symbol: symbol.into(),
            interval: "1day".to_string(),
            output_size,
        });
    }

    /// Request for weekly series
    pub fn weekly_series(&mut self, symbol: impl Into<String>, output_size: u32) -> () {
        self.requests.push(TDRequest {
            symbol: symbol.into(),
            interval: "1week".to_string(),
            output_size,
        });
    }

    /// Request for monthly series
    pub fn monthly_series(&mut self, symbol: impl Into<String>, output_size: u32) -> () {
        self.requests.push(TDRequest {
            symbol: symbol.into(),
            interval: "1month".to_string(),
            output_size,
        });
    }
}

impl Publisher for Twelvedata {
    fn create_endpoint(&mut self) -> MarketResult<()> {
        let base_url = Url::parse(BASE_URL)?;
        self.endpoints = self
            .requests
            .iter()
            .map(|request| {
                let constructed_url = base_url
                    .join(&format!(
                        "?symbol={}&interval={}&outputsize={}&format=json&apikey={}",
                        request.symbol, request.interval, request.output_size, self.token
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

            let prices: TwelvedataPrices = serde_json::from_str(&body)?;
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

            let prices: TwelvedataPrices = serde_json::from_str(&body)?;
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
        for data in self.data.iter() {
            let parsed_data = transform(data);
            result.push(parsed_data)
        }

        // self.data have to be consumed once the data is transformed to MarketSeries
        self.data.clear();
        result
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TwelvedataPrices {
    #[allow(dead_code)]
    meta: MetaData,
    #[serde(rename(deserialize = "values"))]
    time_series: Vec<TimeSeriesData>,
    status: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct MetaData {
    symbol: String,
    interval: String,
    #[allow(dead_code)]
    currency: String,
    #[allow(dead_code)]
    exchange_timezone: String,
    #[allow(dead_code)]
    exchange: String,
    #[allow(dead_code)]
    mic_code: String,
    #[allow(dead_code)]
    r#type: String,
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

fn transform(data: &TwelvedataPrices) -> MarketResult<MarketSeries> {
    // validate the data, first check is status
    if data.status != "ok".to_string() {
        return Err(MarketError::DownloadedData(format!(
            "Downloaded data status is: {}",
            data.status
        )));
    }

    let mut data_series: Vec<Series> = Vec::with_capacity(data.time_series.len());

    for series in data.time_series.iter() {
        let open: f32 =
            series.open.trim().parse().map_err(|e| {
                MarketError::ParsingError(format!("Unable to parse Open field: {}", e))
            })?;
        let close: f32 = series.close.trim().parse().map_err(|e| {
            MarketError::ParsingError(format!("Unable to parse Close field: {}", e))
        })?;
        let high: f32 =
            series.high.trim().parse().map_err(|e| {
                MarketError::ParsingError(format!("Unable to parse High field: {}", e))
            })?;
        let low: f32 =
            series.low.trim().parse().map_err(|e| {
                MarketError::ParsingError(format!("Unable to parse Low field: {}", e))
            })?;
        let volume: f32 = series.volume.trim().parse().map_err(|e| {
            MarketError::ParsingError(format!("Unable to parse Volume field: {}", e))
        })?;
        let date = match data.meta.interval.as_str() {
            "1day" | "1week" | "1month" => NaiveDate::parse_from_str(&series.datetime, "%Y-%m-%d")
                .map_err(|e| {
                    MarketError::ParsingError(format!("Unable to parse Volume field: {}", e))
                })?,
            _ => NaiveDate::parse_from_str(&series.datetime, "%Y-%m-%d %H:%M:%S").map_err(|e| {
                MarketError::ParsingError(format!("Unable to parse Volume field: {}", e))
            })?,
        };

        data_series.push(Series {
            date,
            open,
            close,
            high,
            low,
            volume,
        })
    }

    // sort the series by date
    data_series.sort_by_key(|item| item.date);

    Ok(MarketSeries {
        symbol: data.meta.symbol.clone(),
        interval: data.meta.interval.into(),
        data: data_series,
    })
}
