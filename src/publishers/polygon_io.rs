//! Fetch time series stock data from [Polygon.io](https://polygon.io/docs/stocks/get_v2_aggs_ticker__stocksticker__range__multiplier___timespan___from___to)
///
///Claim your [API Key](https://polygon.io/pricing)
///
/// Example Daily requests:
/// https://api.polygon.io/v2/aggs/ticker/AAPL/range/1/day/2023-01-09/2024-01-09?adjusted=true&sort=asc&limit=120&apiKey=<your_api_key>
///
/// Example Intraday requests:
/// https://api.polygon.io/v2/aggs/ticker/AAPL/range/1/hour/2023-01-09/2024-01-09?adjusted=true&sort=asc&limit=120&apiKey=<your_api_key>
///  
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    client::{Interval, MarketSeries, Series},
    errors::{MarketError, MarketResult},
    publishers::Publisher,
    rest_call::Client,
};

const BASE_URL: &str = "https://api.polygon.io/v2/aggs/ticker/";

/// Fetch time series stock data from [Polygon.io](), implements Publisher trait
#[derive(Debug, Default)]
pub struct Polygon {
    token: String,
    requests: Vec<PolygonRequest>,
    endpoints: Vec<url::Url>,
    data: Vec<PolygonPrices>,
    // interval should be maintained, as it is necesarry in transformation phase
    interval: Interval,
}

#[derive(Debug, Default)]
pub struct PolygonRequest {
    symbol: String,
    // The size of the time window.
    timespan: String,
    // The size of the timespan multiplier.
    multiplier: i32,
    // The start of the aggregate time window (YYYY-MM-DD)
    from_date: String,
    // The end of the aggregate time window (YYYY-MM-DD)
    to_date: String,
    // Limits the number of base aggregates queried to create the aggregate results. Max 50000 and Default 5000
    limit: i32,
}

impl Polygon {
    /// create new instance of Twelvedata
    pub fn new(token: impl Into<String>) -> Self {
        Polygon {
            token: token.into(),
            ..Default::default()
        }
    }

    /// Request for intraday series
    /// it supports only the following intervals: 1min, 5min, 15min, 30min, 1h, 2h, 4h
    pub fn intraday_series(
        &mut self,
        symbol: impl Into<String>,
        from_date: impl Into<String>,
        to_date: impl Into<String>,
        interval: Interval,
        limit: i32,
    ) -> MarketResult<()> {
        let (timespan, multiplier) = match interval {
            Interval::Min1 => ("minute", 1),
            Interval::Min5 => ("minute", 5),
            Interval::Min15 => ("minute", 15),
            Interval::Min30 => ("minute", 30),
            Interval::Hour1 => ("hour", 1),
            Interval::Hour2 => ("hour", 2),
            Interval::Hour4 => ("hour", 4),
            _ => Err(MarketError::UnsuportedInterval(format!(
                "{} Unsuported Interval",
                interval
            )))?,
        };
        self.interval = interval;
        self.requests.push(PolygonRequest {
            symbol: symbol.into(),
            timespan: timespan.into(),
            multiplier,
            from_date: from_date.into(),
            to_date: to_date.into(),
            limit,
        });
        Ok(())
    }

    /// Request for daily series
    pub fn daily_series(
        &mut self,
        symbol: impl Into<String>,
        from_date: impl Into<String>,
        to_date: impl Into<String>,
        limit: i32,
    ) -> () {
        self.interval = Interval::Daily;
        self.requests.push(PolygonRequest {
            symbol: symbol.into(),
            timespan: String::from("day"),
            multiplier: 1,
            from_date: from_date.into(),
            to_date: to_date.into(),
            limit,
        });
    }

    /// Request for weekly series
    pub fn weekly_series(
        &mut self,
        symbol: impl Into<String>,
        from_date: impl Into<String>,
        to_date: impl Into<String>,
        limit: i32,
    ) -> () {
        self.interval = Interval::Weekly;
        self.requests.push(PolygonRequest {
            symbol: symbol.into(),
            timespan: String::from("week"),
            multiplier: 1,
            from_date: from_date.into(),
            to_date: to_date.into(),
            limit,
        });
    }

    /// Request for monthly series
    pub fn monthly_series(
        &mut self,
        symbol: impl Into<String>,
        from_date: impl Into<String>,
        to_date: impl Into<String>,
        limit: i32,
    ) -> () {
        self.interval = Interval::Monthly;
        self.requests.push(PolygonRequest {
            symbol: symbol.into(),
            timespan: String::from("month"),
            multiplier: 1,
            from_date: from_date.into(),
            to_date: to_date.into(),
            limit,
        });
    }
}

impl Publisher for Polygon {
    fn create_endpoint(&mut self) -> MarketResult<()> {
        let base_url = Url::parse(BASE_URL)?;
        self.endpoints = self
            .requests
            .iter()
            .map(|request| {
                let constructed_url = base_url
                    .join(&format!(
                        "{}/range/{}/{}/{}/{}?sort=asc&limit={}&apiKey={}",
                        request.symbol,
                        request.multiplier,
                        request.timespan,
                        request.from_date,
                        request.to_date,
                        request.limit,
                        self.token,
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

            let prices: PolygonPrices = serde_json::from_str(&body)?;
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

            let prices: PolygonPrices = serde_json::from_str(&body)?;
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
            let parsed_data = transform(data, self.interval.clone());
            result.push(parsed_data)
        }

        // self.data have to be consumed once the data is transformed to MarketSeries
        self.data.clear();
        result
    }
}

// PolygonPrices is the struct returned by Polygon.io on Aggregates API
#[derive(Debug, Serialize, Deserialize)]
struct PolygonPrices {
    // Whether or not this response was adjusted for splits.
    adjusted: bool,
    // If present, this value can be used to fetch the next page of data.
    next_url: Option<String>,
    // The number of aggregates (minute or day) used to generate the response.
    #[serde(rename = "queryCount")]
    query_count: i32,
    // A request id assigned by the server.
    request_id: String,
    // The total number of results for this request.
    #[serde(rename = "results")]
    time_series: Vec<TimeSeriesData>,
    #[serde(rename = "resultsCount")]
    results_count: i32,
    // The status of this request's response.
    status: String,
    // The exchange symbol that this item is traded under.
    ticker: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TimeSeriesData {
    // The close price for the symbol in the given time period.
    c: f32,
    // The highest price for the symbol in the given time period.
    h: f32,
    // The lowest price for the symbol in the given time period.
    l: f32,
    // The number of transactions in the aggregate window.
    n: i32,
    // The open price for the symbol in the given time period.
    o: f32,
    // The Unix Msec timestamp for the start of the aggregate window.
    t: i64,
    // The trading volume of the symbol in the given time period.
    v: f64,
    // The volume weighted average price.
    vw: f32,
}

fn transform(data: &PolygonPrices, interval: Interval) -> MarketResult<MarketSeries> {
    // validate the data, first check is status
    if data.status != "OK".to_string() {
        return Err(MarketError::DownloadedData(format!(
            "Downloaded data status is: {}",
            data.status
        )));
    }

    let mut data_series: Vec<Series> = Vec::with_capacity(data.time_series.len());

    for series in data.time_series.iter() {
        let open: f32 = series.o;
        let close: f32 = series.c;
        let high: f32 = series.h;
        let low: f32 = series.l;
        let volume: f32 = series.v as f32;

        // Create a NaiveDateTime from the Unix timestamp
        let datetime = DateTime::from_timestamp_millis(series.t).ok_or(
            MarketError::ParsingError(format!("Unable to parse the timestamp")),
        )?;

        // Extract the date part
        let date = datetime.date_naive();

        data_series.push(Series {
            date,
            open,
            close,
            high,
            low,
            volume,
        })
    }

    // sort the series by date - No needed as it is coming already sorted
    // data_series.sort_by_key(|item| item.date);

    Ok(MarketSeries {
        symbol: data.ticker.clone(),
        interval: interval,
        data: data_series,
    })
}
