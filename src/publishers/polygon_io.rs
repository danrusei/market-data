//! Fetch time series stock data from [Polygon.io](https://polygon.io/docs/stocks/get_v2_aggs_ticker__stocksticker__range__multiplier___timespan___from___to)

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    client::{Interval, MarketSeries, Series},
    errors::{MarketError, MarketResult},
    publishers::Publisher,
};

const BASE_URL: &str = "https://api.polygon.io/v2/aggs/ticker/";

/// Fetch time series stock data from [Polygon.io](), implements Publisher trait
#[derive(Debug)]
pub struct Polygon {
    token: String,
}

#[derive(Debug, Clone)]
pub struct PolygonRequest {
    symbol: String,
    timespan: String,
    multiplier: i32,
    from_date: String,
    to_date: String,
    limit: i32,
    interval: Interval,
}

impl Polygon {
    /// create new instance of Polygon
    pub fn new(token: impl Into<String>) -> Self {
        Polygon {
            token: token.into(),
        }
    }

    /// Request for intraday series
    pub fn intraday_series(
        &self,
        symbol: impl Into<String>,
        from_date: impl Into<String>,
        to_date: impl Into<String>,
        interval: Interval,
        limit: i32,
    ) -> MarketResult<PolygonRequest> {
        let (timespan, multiplier) = match interval {
            Interval::Min1 => ("minute", 1),
            Interval::Min5 => ("minute", 5),
            Interval::Min15 => ("minute", 15),
            Interval::Min30 => ("minute", 30),
            Interval::Hour1 => ("hour", 1),
            Interval::Hour2 => ("hour", 2),
            Interval::Hour4 => ("hour", 4),
            _ => {
                return Err(MarketError::UnsuportedInterval(format!(
                    "{} interval is not supported by Polygon",
                    interval
                )))
            }
        };
        Ok(PolygonRequest {
            symbol: symbol.into(),
            timespan: timespan.into(),
            multiplier,
            from_date: from_date.into(),
            to_date: to_date.into(),
            limit,
            interval,
        })
    }

    /// Request for daily series
    pub fn daily_series(
        &self,
        symbol: impl Into<String>,
        from_date: impl Into<String>,
        to_date: impl Into<String>,
        limit: i32,
    ) -> PolygonRequest {
        PolygonRequest {
            symbol: symbol.into(),
            timespan: "day".to_string(),
            multiplier: 1,
            from_date: from_date.into(),
            to_date: to_date.into(),
            limit,
            interval: Interval::Daily,
        }
    }

    /// Request for weekly series
    pub fn weekly_series(
        &self,
        symbol: impl Into<String>,
        from_date: impl Into<String>,
        to_date: impl Into<String>,
        limit: i32,
    ) -> PolygonRequest {
        PolygonRequest {
            symbol: symbol.into(),
            timespan: "week".to_string(),
            multiplier: 1,
            from_date: from_date.into(),
            to_date: to_date.into(),
            limit,
            interval: Interval::Weekly,
        }
    }

    /// Request for monthly series
    pub fn monthly_series(
        &self,
        symbol: impl Into<String>,
        from_date: impl Into<String>,
        to_date: impl Into<String>,
        limit: i32,
    ) -> PolygonRequest {
        PolygonRequest {
            symbol: symbol.into(),
            timespan: "month".to_string(),
            multiplier: 1,
            from_date: from_date.into(),
            to_date: to_date.into(),
            limit,
            interval: Interval::Monthly,
        }
    }
}

impl Publisher for Polygon {
    type Request = PolygonRequest;

    fn create_endpoint(&self, request: &Self::Request) -> MarketResult<Url> {
        let base_url = Url::parse(BASE_URL)?;
        let mut url = base_url.join(&format!(
            "{}/range/{}/{}/{}/{}",
            request.symbol, request.multiplier, request.timespan, request.from_date, request.to_date,
        ))?;
        {
            let mut pairs = url.query_pairs_mut();
            pairs.append_pair("sort", "asc");
            pairs.append_pair("limit", &request.limit.to_string());
            pairs.append_pair("apiKey", &self.token);
        }
        Ok(url)
    }

    fn transform_data(&self, data: String, request: &Self::Request) -> MarketResult<MarketSeries> {
        let prices: PolygonPrices = serde_json::from_str(&data)?;

        if prices.status != "OK" {
            return Err(MarketError::DownloadedData(format!(
                "Downloaded data status is: {}",
                prices.status
            )));
        }

        let mut data_series: Vec<Series> = Vec::with_capacity(prices.time_series.len());

        for series in prices.time_series.iter() {
            let datetime = DateTime::from_timestamp_millis(series.t).ok_or_else(|| {
                MarketError::ParsingError("Unable to parse the timestamp".to_string())
            })?;

            data_series.push(Series {
                date: datetime.date_naive(),
                open: series.o,
                close: series.c,
                high: series.h,
                low: series.l,
                volume: series.v as f32,
            })
        }

        Ok(MarketSeries {
            symbol: prices.ticker.clone(),
            interval: request.interval.clone(),
            data: data_series,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PolygonPrices {
    #[serde(rename = "results")]
    time_series: Vec<TimeSeriesData>,
    status: String,
    ticker: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TimeSeriesData {
    c: f32,
    h: f32,
    l: f32,
    o: f32,
    t: i64,
    v: f64,
}
