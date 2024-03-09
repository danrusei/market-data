//! Fetch time series stock data from [IEX](https://iexcloud.io/docs/), implements Publisher trait
///
/// Example - retrieves historical data for Apple's daily stock price:
/// Reference https://iexcloud.io/docs/api/#historical-prices
/// https://api.iex.cloud/v1/stock/AAPL/chart/2y?token=YOUR-TOKEN-HERE
///
/// Reference https://iexcloud.io/docs/core/HISTORICAL_PRICES
/// https://api.iex.cloud/v1/data/core/historical_prices/aapl?range=2y&token=YOUR-TOKEN-HERE
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    client::{MarketSeries, Series},
    errors::{MarketError, MarketResult},
    publishers::Publisher,
    rest_call::Client,
};

const BASE_URL: &str = "https://api.iex.cloud/v1/stock/";

/// Fetch time series stock data from [IEX](https://iexcloud.io/docs/), implements Publisher trait
#[derive(Debug, Default)]
pub struct Iex {
    token: String,
    requests: Vec<IexRequest>,
    endpoints: Vec<url::Url>,
    data: Vec<Vec<IexDailyPrices>>,
}

#[derive(Debug, Default)]
pub struct IexRequest {
    symbol: String,
    range: String,
}

impl Iex {
    pub fn new(token: String) -> Self {
        Iex {
            token: token,
            ..Default::default()
        }
    }

    /// Request for daily series
    pub fn daily_series(&mut self, symbol: String, range: String) -> () {
        self.requests.push(IexRequest { symbol, range });
    }
}

impl Publisher for Iex {
    fn create_endpoint(&mut self) -> MarketResult<()> {
        let base_url = Url::parse(BASE_URL)?;
        self.endpoints = self
            .requests
            .iter()
            .map(|request| {
                let constructed_url = base_url
                    .join(&format!(
                        "{}/chart/{}?token={}",
                        request.symbol, request.range, self.token
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

            let prices: Vec<IexDailyPrices> = serde_json::from_str(&body)?;
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

            let prices: Vec<IexDailyPrices> = serde_json::from_str(&body)?;
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
        let mut result: Vec<MarketResult<MarketSeries>> = Vec::new();
        let mut symbol = String::new();
        for data in self.data.iter() {
            let mut data_series: Vec<Series> = Vec::with_capacity(data.len());
            for series in data.iter() {
                let date: NaiveDate = match NaiveDate::parse_from_str(&series.date, "%Y-%m-%d") {
                    //.map_err(|e| {MarketError::ParsingError(format!("Unable to parse Date field: {}", e))
                    Ok(date) => date,
                    Err(err) => {
                        result.push(Err(MarketError::ParsingError(format!(
                            "Unable to parse Date field: {}",
                            err
                        ))));
                        break;
                    }
                };
                data_series.push(Series {
                    date: date,
                    open: series.open,
                    close: series.close,
                    high: series.high,
                    low: series.low,
                    volume: series.volume as f32,
                });
                symbol = series.symbol.clone();
            }

            // sort the series by date
            data_series.sort_by_key(|item| item.date);

            result.push(Ok(MarketSeries {
                symbol: symbol.clone(),
                interval: "Daily".to_string(),
                data: data_series,
            }))
        }
        result
    }
}

#[derive(Debug, Deserialize, Serialize)]
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
