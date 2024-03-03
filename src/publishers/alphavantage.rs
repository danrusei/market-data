// Retrieve time series stock data from:
// https://www.alphavantage.co/documentation/#time-series-data
//
// Claim your API Key
// https://www.alphavantage.co/support/#api-key
//
// Example Daily requests:
// https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol=IBM&apikey=demo
// https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol=IBM&outputsize=full&apikey=demo
//
// Example intraday requests:
// The API will return the most recent 100 intraday OHLCV bars by default when the outputsize parameter is not set
// https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol=IBM&interval=5min&apikey=demo
// Query the most recent full 30 days of intraday data by setting outputsize=full
// https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol=IBM&interval=5min&outputsize=full&apikey=demo

use serde::Deserialize;
use std::collections::HashMap;
use url::Url;

use crate::{
    client::{MarketData, Series},
    errors::MarketResult,
    publishers::Publisher,
    rest_call::Client,
    MarketError,
};

const BASE_URL: &str = "https://www.alphavantage.co/";

#[derive(Debug, Default)]
pub struct AlphaVantage {
    token: String,
    symbol: String,
    function: Function,
    output_size: OutputSize,
    endpoint: Option<url::Url>,
    data: Option<AlphaDailyPrices>,
}

#[derive(Debug, Default, PartialEq)]
pub enum Function {
    // https://www.alphavantage.co/documentation/#daily
    #[default]
    TimeSeriesDaily,
    //https://www.alphavantage.co/documentation/#dailyadj
    #[allow(dead_code)]
    TimeSeriesDailyAdjusted,
    // https://www.alphavantage.co/documentation/#intraday
    #[allow(dead_code)]
    TimeSeriesIntraday,
    //https://www.alphavantage.co/documentation/#weekly
    #[allow(dead_code)]
    TimeSeriesWeekly,
    //https://www.alphavantage.co/documentation/#weeklyadj
    #[allow(dead_code)]
    TimeSeriesWeeklyAdjusted,
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
    pub fn new(token: String) -> Self {
        AlphaVantage {
            token: token,
            ..Default::default()
        }
    }
    pub fn for_daily_series(&mut self, symbol: String, output_size: OutputSize) -> () {
        self.symbol = symbol;
        self.function = Function::TimeSeriesDaily;
        self.output_size = output_size;
    }
    pub fn for_intraday_series(&mut self, _symbol: String) -> () {
        todo!("not supported yet")
    }
    pub fn for_weekly_series(&mut self, _symbol: String) -> () {
        todo!("not supported yet")
    }
}

impl Publisher for AlphaVantage {
    fn create_endpoint(&mut self) -> MarketResult<()> {
        let base_url = Url::parse(BASE_URL)?;
        let constructed_url = base_url.join(&format!(
            "query?function={}&symbol={}&outputsize={}&datatype=json&apikey={}",
            self.function.to_string(),
            self.symbol,
            self.output_size.to_string(),
            self.token
        ))?;
        self.endpoint = dbg!(Some(constructed_url));
        Ok(())
    }

    fn get_data(&mut self) -> MarketResult<()> {
        let client = Client::new(
            self.endpoint
                .clone()
                .expect("Use create_endpoint method first to construct the URL"),
        );
        let response = client.get_data()?;
        let body = dbg!(response.text()?);

        let prices: AlphaDailyPrices = serde_json::from_str(&body)?;
        self.data = Some(prices);

        Ok(())
    }

    fn transform_data(&self) -> MarketResult<MarketData> {
        if let Some(data) = self.data.as_ref() {
            let mut data_series: Vec<Series> = Vec::new();
            for (date, series) in data.time_series.iter() {
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
                let volume: f32 = series.volume.trim().parse().map_err(|e| {
                    MarketError::ParsingError(format!("Unable to parse Volume field: {}", e))
                })?;

                data_series.push(Series {
                    date: date.to_owned(),
                    open: open,
                    close: close,
                    high: high,
                    low: low,
                    volume: volume,
                })
            }
            Ok(MarketData {
                symbol: self.symbol.clone(),
                data: data_series,
            })
        } else {
            Err(MarketError::DownloadedData(
                "No data downloaded".to_string(),
            ))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AlphaDailyPrices {
    #[serde(flatten)]
    pub meta_data: MetaData,
    #[serde(rename = "Time Series (Daily)")]
    pub time_series: HashMap<String, TimeSeriesData>,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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

impl ToString for Function {
    fn to_string(&self) -> String {
        match self {
            Function::TimeSeriesDaily => String::from("TIME_SERIES_DAILY"),
            Function::TimeSeriesDailyAdjusted => String::from("TIME_SERIES_DAILY_ADJUSTED"),
            Function::TimeSeriesIntraday => String::from("TIME_SERIES_INTRADAY"),
            Function::TimeSeriesWeekly => String::from("TIME_SERIES_WEEKLY"),
            Function::TimeSeriesWeeklyAdjusted => String::from("TIME_SERIES_WEEKLY_ADJUSTED"),
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
