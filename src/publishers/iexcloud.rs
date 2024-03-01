//Retrieve market data from:
//
//https://iexcloud.io/docs/
//
//Example - retrieves historical data for Apple's daily stock price:
// https://api.iex.cloud/v1/stock/AAPL/chart/2y?token=YOUR-TOKEN-HERE
//  --> reference https://iexcloud.io/docs/api/#historical-prices
//
// https://api.iex.cloud/v1/data/core/historical_prices/aapl?range=2y&token=YOUR-TOKEN-HERE
// --> reference https://iexcloud.io/docs/core/HISTORICAL_PRICES

use serde::Deserialize;
use url::Url;

use crate::{
    client::Client,
    errors::MarketResult,
    publishers::{DataRetrieval, MarketData},
};

const BASE_URL: &str = "https://api.iex.cloud/v1/stock/";

#[derive(Debug, Default)]
pub struct Iex {
    token: String,
    symbol: String,
    range: String,
    endpoint: Option<url::Url>,
    data: Option<HistoricalPrices>,
}

impl Iex {
    pub fn new(token: String, symbol: String, range: String) -> Self {
        Iex {
            token: token,
            symbol: symbol,
            range: range,
            ..Default::default()
        }
    }
}

impl DataRetrieval for Iex {
    fn create_endpoint(&mut self) -> MarketResult<()> {
        let base_url = Url::parse(BASE_URL)?;
        let constructed_url = base_url.join(&format!(
            "{}/chart/{}?token={}",
            self.symbol, self.range, self.token
        ))?;
        self.endpoint = Some(constructed_url);
        Ok(())
    }

    async fn get_data(&mut self) -> MarketResult<()> {
        let client = Client::new(
            self.endpoint
                .clone()
                .expect("Use create_endpoint method first to construct the URL"),
        );
        let response = client.get_data().await?;
        let body = response.text().await?;

        let prices: HistoricalPrices = serde_json::from_str(&body)?;

        //self.data = Some(prices);
        println!("Response body deserialized: {:?}", prices);

        Ok(())
    }

    fn transform_data(&self) -> MarketData {
        todo!()
    }
}

#[derive(Debug, Deserialize)]
struct HistoricalPrices {
    prices: Vec<HistoricalPrice>,
}

#[derive(Debug, Deserialize)]
struct HistoricalPrice {
    close: f32,
    high: f32,
    low: f32,
    open: f32,
    symbol: String,
    volume: u64,
    id: String,
    key: String,
    subkey: String,
    date: String,
    updated: u64,
    #[serde(rename(deserialize = "changeOverTime"))]
    change_over_time: f32,
    #[serde(rename(deserialize = "marketChangeOverTime"))]
    market_change_over_time: f32,
    #[serde(rename(deserialize = "uOpen"))]
    u_open: f32,
    #[serde(rename(deserialize = "uClose"))]
    u_close: f32,
    #[serde(rename(deserialize = "uHigh"))]
    u_high: f32,
    #[serde(rename(deserialize = "uLow"))]
    u_low: f32,
    #[serde(rename(deserialize = "uVolume"))]
    u_volume: u64,
    #[serde(rename(deserialize = "fOpen"))]
    f_open: f32,
    #[serde(rename(deserialize = "fClose"))]
    f_close: f32,
    #[serde(rename(deserialize = "fHigh"))]
    f_high: f32,
    #[serde(rename(deserialize = "fLow"))]
    f_low: f32,
    #[serde(rename(deserialize = "fVolume"))]
    f_volume: u64,
    label: String,
    change: f32,
    #[serde(rename(deserialize = "changePercent"))]
    change_percent: f32,
}
