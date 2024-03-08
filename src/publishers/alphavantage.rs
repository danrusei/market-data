//! Fetch time series stock data from [AlphaVantage](https://www.alphavantage.co/documentation/#time-series-data)
///
/// Claim your [API Key](https://www.alphavantage.co/support/#api-key)
//
/// Example Daily requests:
/// https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol=IBM&apikey=demo
/// https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol=IBM&outputsize=full&apikey=demo
///
/// Example intraday requests:
/// The API will return the most recent 100 intraday OHLCV bars by default when the outputsize parameter is not set
/// https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol=IBM&interval=5min&apikey=demo
/// Query the most recent full 30 days of intraday data by setting outputsize=full
/// https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol=IBM&interval=5min&outputsize=full&apikey=demo
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

use crate::{
    client::{MarketSeries, Series},
    errors::MarketResult,
    publishers::Publisher,
    rest_call::Client,
    MarketError,
};

const BASE_URL: &str = "https://www.alphavantage.co/";

/// Fetch time series stock data from [AlphaVantage](https://www.alphavantage.co/documentation/#time-series-data),
/// implements Publisher trait
#[derive(Debug, Default)]
pub struct AlphaVantage {
    token: String,
    requests: Vec<AVRequest>,
    endpoints: Vec<url::Url>,
    data: Vec<AlphaDailyPrices>,
}

#[derive(Debug, Default)]
pub struct AVRequest {
    symbol: String,
    function: Function,
    interval: Option<Interval>,
    output_size: OutputSize,
}

#[derive(Debug, PartialEq)]
pub enum Interval {
    Min1,
    Min5,
    Min15,
    Min30,
    Min60,
}

#[derive(Debug, Default, PartialEq)]
pub enum Function {
    // https://www.alphavantage.co/documentation/#intraday
    TimeSeriesIntraday,
    // https://www.alphavantage.co/documentation/#daily
    #[default]
    TimeSeriesDaily,
    //https://www.alphavantage.co/documentation/#dailyadj
    TimeSeriesDailyAdjusted,
    //https://www.alphavantage.co/documentation/#weekly
    TimeSeriesWeekly,
    //https://www.alphavantage.co/documentation/#weeklyadj
    TimeSeriesWeeklyAdjusted,
    //https://www.alphavantage.co/documentation/#monthly
    TimeSeriesMonthly,
    //https://www.alphavantage.co/documentation/#monthlyadj
    TimeSeriesMonthlyAdjusted,
}

#[derive(Debug, Default, PartialEq)]
pub enum OutputSize {
    // compact returns only the latest 100 data points
    #[default]
    Compact,
    // full returns the full-length time series of 20+ years of historical data on Daily requests
    // and trailing 30 days of the most recent intraday for Intraday Series
    Full,
}

impl AlphaVantage {
    /// create new instance of AlphaVantage
    pub fn new(token: String) -> Self {
        AlphaVantage {
            token: token,
            ..Default::default()
        }
    }

    /// Request for intraday series
    pub fn intraday_series(
        &mut self,
        symbol: String,
        output_size: OutputSize,
        interval: Interval,
    ) -> () {
        let function = Function::TimeSeriesIntraday;
        self.requests.push(AVRequest {
            symbol,
            function,
            interval: Some(interval),
            output_size,
        });
    }

    /// Request for daily series
    pub fn daily_series(&mut self, symbol: String, output_size: OutputSize) -> () {
        let function = Function::TimeSeriesDaily;
        self.requests.push(AVRequest {
            symbol,
            function,
            interval: None,
            output_size,
        });
    }

    /// Request for weekly series
    pub fn weekly_series(&mut self, symbol: String, output_size: OutputSize) -> () {
        let function = Function::TimeSeriesWeekly;
        self.requests.push(AVRequest {
            symbol,
            function,
            interval: None,
            output_size,
        });
    }

    /// Request for monthly series
    pub fn monthly_series(&mut self, symbol: String, output_size: OutputSize) -> () {
        let function = Function::TimeSeriesMonthly;
        self.requests.push(AVRequest {
            symbol,
            function,
            interval: None,
            output_size,
        });
    }
}

impl Publisher for AlphaVantage {
    fn create_endpoint(&mut self) -> MarketResult<()> {
        let base_url = Url::parse(BASE_URL)?;
        self.endpoints = self
            .requests
            .iter()
            .map(|request| {
                let constructed_url = match request.function {
                    Function::TimeSeriesIntraday => {
                        todo!()
                    }
                    _ => base_url
                        .join(&format!(
                            "query?function={}&symbol={}&outputsize={}&datatype=json&apikey={}",
                            request.function.to_string(),
                            request.symbol,
                            request.output_size.to_string(),
                            self.token
                        ))
                        .unwrap(),
                };
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

            let prices: AlphaDailyPrices = serde_json::from_str(&body)?;
            self.data.push(prices);
        }

        // self.endpoints have to be consumed once the data was downloaded for requested URL
        self.endpoints.clear();

        Ok(())
    }

    #[cfg(feature = "use-async")]
    async fn get_data(&mut self) -> MarketResult<()> {
        let client = Client::new(
            self.endpoint
                .clone()
                .expect("Use create_endpoint method first to construct the URL"),
        );
        for endpoint in &self.endpoints {
            let response = client.get_data().await?;
            let body = response.text().await?;

            let prices: AlphaDailyPrices = serde_json::from_str(&body)?;
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
pub struct AlphaDailyPrices {
    #[serde(rename = "Meta Data")]
    pub meta_data: MetaData,
    #[serde(rename = "Time Series (Daily)")]
    pub time_series: HashMap<String, TimeSeriesData>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MetaData {
    #[serde(rename = "1. Information")]
    pub information: String,
    #[serde(rename = "2. Symbol")]
    pub symbol: String,
    #[serde(rename = "3. Last Refreshed")]
    pub last_refreshed: String,
    #[serde(rename = "4. Output Size")]
    pub output_size: String,
    #[serde(rename = "5. Time Zone")]
    pub time_zone: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeSeriesData {
    #[serde(rename = "1. open")]
    pub open: String,
    #[serde(rename = "2. high")]
    pub high: String,
    #[serde(rename = "3. low")]
    pub low: String,
    #[serde(rename = "4. close")]
    pub close: String,
    #[serde(rename = "5. volume")]
    pub volume: String,
}

fn transform(data: &AlphaDailyPrices) -> MarketResult<MarketSeries> {
    let mut data_series: Vec<Series> = Vec::with_capacity(data.time_series.len());

    for (date, series) in data.time_series.iter() {
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
        let date: NaiveDate = NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|e| {
            MarketError::ParsingError(format!("Unable to parse Volume field: {}", e))
        })?;

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
        symbol: data.meta_data.symbol.clone(),
        interval: "unknown".to_string(),
        data: data_series,
    })
}

impl ToString for Function {
    fn to_string(&self) -> String {
        match self {
            Function::TimeSeriesIntraday => String::from("TIME_SERIES_INTRADAY"),
            Function::TimeSeriesDaily => String::from("TIME_SERIES_DAILY"),
            Function::TimeSeriesDailyAdjusted => String::from("TIME_SERIES_DAILY_ADJUSTED"),
            Function::TimeSeriesWeekly => String::from("TIME_SERIES_WEEKLY"),
            Function::TimeSeriesWeeklyAdjusted => String::from("TIME_SERIES_WEEKLY_ADJUSTED"),
            Function::TimeSeriesMonthly => String::from("TIME_SERIES_MONTHLY"),
            Function::TimeSeriesMonthlyAdjusted => String::from("TIME_SERIES_MONTHLY_ADJUSTED"),
        }
    }
}
impl ToString for OutputSize {
    fn to_string(&self) -> String {
        match self {
            OutputSize::Compact => String::from("Compact"),
            OutputSize::Full => String::from("Full"),
        }
    }
}

impl ToString for Interval {
    fn to_string(&self) -> String {
        match self {
            Interval::Min1 => String::from("1min"),
            Interval::Min5 => String::from("5min"),
            Interval::Min15 => String::from("15min"),
            Interval::Min30 => String::from("30min"),
            Interval::Min60 => String::from("60min"),
        }
    }
}
