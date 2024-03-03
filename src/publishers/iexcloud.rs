// Retrieve time series stock data from:
// https://iexcloud.io/docs/
//
// Example - retrieves historical data for Apple's daily stock price:
// Reference https://iexcloud.io/docs/api/#historical-prices
// https://api.iex.cloud/v1/stock/AAPL/chart/2y?token=YOUR-TOKEN-HERE
//
// Reference https://iexcloud.io/docs/core/HISTORICAL_PRICES
// https://api.iex.cloud/v1/data/core/historical_prices/aapl?range=2y&token=YOUR-TOKEN-HERE

use serde::Deserialize;
use url::Url;

use crate::{
    client::{MarketData, Series},
    errors::{MarketError, MarketResult},
    publishers::Publisher,
    rest_call::Client,
};

const BASE_URL: &str = "https://api.iex.cloud/v1/stock/";

#[derive(Debug, Default)]
pub struct Iex {
    token: String,
    symbol: String,
    range: String,
    endpoint: Option<url::Url>,
    data: Option<Vec<IexDailyPrices>>,
}

impl Iex {
    pub fn new(token: String) -> Self {
        Iex {
            token: token,
            ..Default::default()
        }
    }
    pub fn for_series(&mut self, symbol: String, range: String) -> () {
        self.symbol = symbol;
        self.range = range;
    }
}

impl Publisher for Iex {
    fn create_endpoint(&mut self) -> MarketResult<()> {
        let base_url = Url::parse(BASE_URL)?;
        let constructed_url = base_url.join(&format!(
            "{}/chart/{}?token={}",
            self.symbol, self.range, self.token
        ))?;
        self.endpoint = Some(constructed_url);
        Ok(())
    }

    fn get_data(&mut self) -> MarketResult<()> {
        let client = Client::new(
            self.endpoint
                .clone()
                .expect("Use create_endpoint method first to construct the URL"),
        );
        let response = client.get_data()?;
        let body = response.text()?;

        let prices: Vec<IexDailyPrices> = serde_json::from_str(&body)?;
        self.data = Some(prices);

        Ok(())
    }

    fn transform_data(&self) -> MarketResult<MarketData> {
        if let Some(data) = self.data.as_ref() {
            let data_series: Vec<Series> = data
                .iter()
                .map(|f| Series {
                    date: f.date.clone(),
                    open: f.open,
                    close: f.close,
                    high: f.high,
                    low: f.low,
                    volume: f.volume as f32,
                })
                .collect();

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
struct IexDailyPrices {
    close: f32,
    high: f32,
    low: f32,
    open: f32,
    #[allow(dead_code)]
    #[serde(rename(deserialize = "priceDate"))]
    price_date: String,
    #[allow(dead_code)]
    symbol: String,
    volume: u64,
    #[allow(dead_code)]
    id: String,
    #[allow(dead_code)]
    key: String,
    #[allow(dead_code)]
    subkey: String,
    date: String,
    #[allow(dead_code)]
    updated: u64,
    #[allow(dead_code)]
    #[serde(rename(deserialize = "changeOverTime"))]
    change_over_time: f32,
    #[allow(dead_code)]
    #[serde(rename(deserialize = "marketChangeOverTime"))]
    market_change_over_time: f32,
    #[allow(dead_code)]
    #[serde(rename(deserialize = "uOpen"))]
    u_open: f32,
    #[allow(dead_code)]
    #[serde(rename(deserialize = "uClose"))]
    u_close: f32,
    #[allow(dead_code)]
    #[serde(rename(deserialize = "uHigh"))]
    u_high: f32,
    #[allow(dead_code)]
    #[serde(rename(deserialize = "uLow"))]
    u_low: f32,
    #[allow(dead_code)]
    #[serde(rename(deserialize = "uVolume"))]
    u_volume: u64,
    #[allow(dead_code)]
    #[serde(rename(deserialize = "fOpen"))]
    f_open: f32,
    #[allow(dead_code)]
    #[serde(rename(deserialize = "fClose"))]
    f_close: f32,
    #[allow(dead_code)]
    #[serde(rename(deserialize = "fHigh"))]
    f_high: f32,
    #[allow(dead_code)]
    #[serde(rename(deserialize = "fLow"))]
    f_low: f32,
    #[allow(dead_code)]
    #[serde(rename(deserialize = "fVolume"))]
    f_volume: u64,
    #[allow(dead_code)]
    label: String,
    #[allow(dead_code)]
    change: f32,
    #[allow(dead_code)]
    #[serde(rename(deserialize = "changePercent"))]
    change_percent: f32,
}

// HistoricalPrice struct fields:
// close    number  Adjusted data for historical dates. Split adjusted only.
// high     number	Adjusted data for historical dates. Split adjusted only.
// low	    number	Adjusted data for historical dates. Split adjusted only.
// open	    number	Adjusted data for historical dates. Split adjusted only.
// symbol	string	Associated symbol or ticker
// volume	number	Adjusted data for historical dates. Split adjusted only.
// changeOverTime	number	Percent change of each interval relative to first value. Useful for comparing multiple stocks.
// marketChangeOverTime	number	Percent change of each interval relative to first value. 15 minute delayed consolidated data.
// uOpen	number	Unadjusted data for historical dates.
// uClose	number	Unadjusted data for historical dates.
// uHigh	number	Unadjusted data for historical dates.
// uLow	    number	Unadjusted data for historical dates.
// uVolume	number	Unadjusted data for historical dates.
// fOpen	number	Fully adjusted for historical dates.
// fClose	number	Fully adjusted for historical dates.
// fHigh	number	Fully adjusted for historical dates.
// fLow	    number	Fully adjusted for historical dates.
// fVolume	number	Fully adjusted for historical dates.
// label	number	A human readable format of the date depending on the range.
// change	number	Change from previous trading day.
// changePercent	number	Change percent from previous trading day.
