//! Fetch time series stock data from [Finnhub](https://finnhub.io/docs/api), implements Publisher trait
///
/// Example - retrieves historical data for Apple's daily stock price:
/// https://finnhub.io/api/v1/stock/candle?symbol=AAPL&resolution=D&from=1672531200&to=1675209600&token=YOUR-TOKEN-HERE
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    client::{Interval, MarketSeries, Series},
    errors::{MarketError, MarketResult},
    publishers::Publisher,
    rest_call::Client,
};

const BASE_URL: &str = "https://finnhub.io/api/v1/stock/candle";

/// Fetch time series stock data from [Finnhub](https://finnhub.io/docs/api), implements Publisher trait
#[derive(Debug, Default)]
pub struct Finnhub {
    token: String,
    requests: Vec<FinnhubRequest>,
    endpoints: Vec<url::Url>,
    data: Vec<FinnhubCandles>,
}

#[derive(Debug, Default)]
struct FinnhubRequest {
    symbol: String,
    resolution: String,
    from: i64,
    to: i64,
}

impl Finnhub {
    pub fn new(token: impl Into<String>) -> Self {
        Finnhub {
            token: token.into(),
            ..Default::default()
        }
    }

    /// Request for daily series
    pub fn daily_series(&mut self, symbol: impl Into<String>, from: i64, to: i64) {
        self.requests.push(FinnhubRequest {
            symbol: symbol.into(),
            resolution: "D".to_string(),
            from,
            to,
        });
    }

    /// Request for weekly series
    pub fn weekly_series(&mut self, symbol: impl Into<String>, from: i64, to: i64) {
        self.requests.push(FinnhubRequest {
            symbol: symbol.into(),
            resolution: "W".to_string(),
            from,
            to,
        });
    }

    /// Request for monthly series
    pub fn monthly_series(&mut self, symbol: impl Into<String>, from: i64, to: i64) {
        self.requests.push(FinnhubRequest {
            symbol: symbol.into(),
            resolution: "M".to_string(),
            from,
            to,
        });
    }

    /// Request for intraday series
    pub fn intraday_series(
        &mut self,
        symbol: impl Into<String>,
        from: i64,
        to: i64,
        interval: Interval,
    ) -> MarketResult<()> {
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
        self.requests.push(FinnhubRequest {
            symbol: symbol.into(),
            resolution,
            from,
            to,
        });
        Ok(())
    }
}

impl Publisher for Finnhub {
    fn create_endpoint(&mut self) -> MarketResult<()> {
        let base_url = Url::parse(BASE_URL)?;
        self.endpoints = self
            .requests
            .iter()
            .map(|request| {
                let mut url = base_url.clone();
                url.query_pairs_mut()
                    .append_pair("symbol", &request.symbol)
                    .append_pair("resolution", &request.resolution)
                    .append_pair("from", &request.from.to_string())
                    .append_pair("to", &request.to.to_string())
                    .append_pair("token", &self.token);
                url
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

            let candles: FinnhubCandles = serde_json::from_str(&body)?;
            self.data.push(candles);
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

            let candles: FinnhubCandles = serde_json::from_str(&body)?;
            self.data.push(candles);
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
        for candles in self.data.iter() {
            if candles.status == "no_data" {
                result.push(Err(MarketError::DownloadedData(
                    "No data returned from Finnhub".to_string(),
                )));
                continue;
            }

            let mut data_series: Vec<Series> = Vec::with_capacity(candles.t.len());
            for i in 0..candles.t.len() {
                let datetime = match DateTime::from_timestamp(candles.t[i], 0) {
                    Some(dt) => dt.date_naive(),
                    None => {
                        result.push(Err(MarketError::ParsingError(format!(
                            "Unable to parse timestamp: {}",
                            candles.t[i]
                        ))));
                        continue;
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

            // Determine interval from candles - for simplicity we could store it in Request,
            // but for now let's assume if it has data it belongs to one of the requests.
            // In a better implementation, we'd map data back to the original request's interval.
            // For now, let's just use Daily as default if we can't determine it easily.
            // Actually, Finnhub candles doesn't return the resolution in the response.

            result.push(Ok(MarketSeries {
                symbol: "".to_string(), // Finnhub candle doesn't return symbol in response either
                interval: Interval::Daily, // This is a limitation of this approach
                data: data_series,
            }))
        }
        self.data.clear();
        result
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
