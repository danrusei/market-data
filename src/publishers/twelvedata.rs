//! Fetch time series stock data from [Twelvedata](https://twelvedata.com/docs#time-series)
//
/// Claim your API Key (Basic Free Account with 800 API credits per day)
/// https://twelvedata.com/pricing
/// https://support.twelvedata.com/en/articles/5615854-credits
/// For instance, if you access /time_series data for AAPL, MSFT, and TSLA - you would consume a total of (1 credit) * (3 symbols) = 3 credits.
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
    client::{MarketSeries, Series},
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
    symbol: String,
    interval: String,
    output_size: u32,
    endpoint: Option<url::Url>,
    data: Option<TwelvedataDailyPrices>,
}

impl Twelvedata {
    pub fn new(token: String) -> Self {
        Twelvedata {
            token: token,
            ..Default::default()
        }
    }
    pub fn for_daily_series(&mut self, symbol: String, output_size: u32) -> () {
        self.symbol = symbol;
        self.interval = "1day".to_string();
        self.output_size = output_size;
    }
    pub fn for_intraday_series(&mut self, _symbol: String) -> () {
        todo!("not supported yet")
    }
    pub fn for_weekly_series(&mut self, _symbol: String) -> () {
        todo!("not supported yet")
    }
}

impl Publisher for Twelvedata {
    fn create_endpoint(&mut self) -> MarketResult<()> {
        let base_url = Url::parse(BASE_URL)?;
        let constructed_url = base_url.join(&format!(
            "?symbol={}&interval={}&outputsize={}&format=json&apikey={}",
            self.symbol, self.interval, self.output_size, self.token
        ))?;
        self.endpoint = Some(constructed_url);
        Ok(())
    }

    #[cfg(feature = "use-sync")]
    fn get_data(&mut self) -> MarketResult<()> {
        let rest_client = Client::new(
            self.endpoint
                .clone()
                .expect("Use create_endpoint method first to construct the URL"),
        );
        let response = rest_client.get_data()?;
        let body = response.into_string()?;

        let prices: TwelvedataDailyPrices = serde_json::from_str(&body)?;
        self.data = Some(prices);

        Ok(())
    }

    #[cfg(feature = "use-async")]
    async fn get_data(&mut self) -> MarketResult<()> {
        let client = Client::new(
            self.endpoint
                .clone()
                .expect("Use create_endpoint method first to construct the URL"),
        );
        let response = client.get_data().await?;
        let body = response.text().await?;

        let prices: TwelvedataDailyPrices = serde_json::from_str(&body)?;
        self.data = Some(prices);

        Ok(())
    }

    fn to_writer(&self, writer: impl std::io::Write) -> MarketResult<()> {
        if let Some(data) = &self.data {
            serde_json::to_writer(writer, data).map_err(|err| {
                MarketError::ToWriter(format!("Unable to write to writer, got the error: {}", err))
            })?;
        }
        Ok(())
    }

    fn transform_data(&self) -> MarketResult<MarketSeries> {
        if let Some(data) = self.data.as_ref() {
            if data.status != "ok".to_string() {
                return Err(MarketError::DownloadedData(format!(
                    "Downloaded data status is: {}",
                    data.status
                )));
            }
            let mut data_series: Vec<Series> = Vec::with_capacity(data.time_series.len());
            for series in data.time_series.iter() {
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
                let date: NaiveDate = NaiveDate::parse_from_str(&series.datetime, "%Y-%m-%d")
                    .map_err(|e| {
                        MarketError::ParsingError(format!("Unable to parse Volume field: {}", e))
                    })?;

                data_series.push(Series {
                    date: date,
                    open: open,
                    close: close,
                    high: high,
                    low: low,
                    volume: volume,
                })
            }

            // sort the series by date
            data_series.sort_by_key(|item| item.date);

            Ok(MarketSeries {
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

#[derive(Debug, Deserialize, Serialize)]
struct TwelvedataDailyPrices {
    #[allow(dead_code)]
    meta: MetaData,
    #[serde(rename(deserialize = "values"))]
    time_series: Vec<TimeSeriesData>,
    status: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct MetaData {
    #[allow(dead_code)]
    symbol: String,
    #[allow(dead_code)]
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
