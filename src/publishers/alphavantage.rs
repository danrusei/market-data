//! Fetch time series stock data from [AlphaVantage](https://www.alphavantage.co/documentation/#time-series-data)

use chrono::NaiveDate;
use url::Url;

use crate::{
    client::{Interval, MarketSeries, Series},
    errors::MarketResult,
    publishers::Publisher,
    MarketError,
};

const BASE_URL: &str = "https://www.alphavantage.co/";

/// Fetch time series stock data from [AlphaVantage](https://www.alphavantage.co/documentation/#time-series-data),
/// implements Publisher trait
#[derive(Debug)]
pub struct AlphaVantage {
    token: String,
}

#[derive(Debug)]
pub struct AVRequest {
    symbol: String,
    function: Function,
    interval: Option<String>,
    output_size: OutputSize,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum Function {
    Intraday,
    #[default]
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum OutputSize {
    #[default]
    Compact,
    Full,
}

impl AlphaVantage {
    /// create new instance of AlphaVantage
    pub fn new(token: impl Into<String>) -> Self {
        AlphaVantage {
            token: token.into(),
        }
    }

    /// Request for intraday series
    pub fn intraday_series(
        &self,
        symbol: impl Into<String>,
        output_size: OutputSize,
        interval: Interval,
    ) -> MarketResult<AVRequest> {
        let interval_str = match interval {
            Interval::Min1 => "1min".to_string(),
            Interval::Min5 => "5min".to_string(),
            Interval::Min15 => "15min".to_string(),
            Interval::Min30 => "30min".to_string(),
            Interval::Hour1 => "60min".to_string(),
            _ => {
                return Err(MarketError::UnsuportedInterval(format!(
                    "{} interval is not supported by AlphaVantage",
                    interval
                )))
            }
        };
        Ok(AVRequest {
            symbol: symbol.into(),
            function: Function::Intraday,
            interval: Some(interval_str),
            output_size,
        })
    }

    /// Request for daily series
    pub fn daily_series(&self, symbol: impl Into<String>, output_size: OutputSize) -> AVRequest {
        AVRequest {
            symbol: symbol.into(),
            function: Function::Daily,
            interval: None,
            output_size,
        }
    }

    /// Request for weekly series
    pub fn weekly_series(&self, symbol: impl Into<String>, output_size: OutputSize) -> AVRequest {
        AVRequest {
            symbol: symbol.into(),
            function: Function::Weekly,
            interval: None,
            output_size,
        }
    }

    /// Request for monthly series
    pub fn monthly_series(&self, symbol: impl Into<String>, output_size: OutputSize) -> AVRequest {
        AVRequest {
            symbol: symbol.into(),
            function: Function::Monthly,
            interval: None,
            output_size,
        }
    }
}

impl Publisher for AlphaVantage {
    type Request = AVRequest;

    fn create_endpoint(&self, request: &Self::Request) -> MarketResult<Url> {
        let base_url = Url::parse(BASE_URL)?;
        let mut url = base_url.join("query")?;
        {
            let mut pairs = url.query_pairs_mut();
            pairs.append_pair("function", &request.function.to_string());
            pairs.append_pair("symbol", &request.symbol);
            pairs.append_pair(
                "outputsize",
                &request.output_size.to_string().to_lowercase(),
            );
            pairs.append_pair("datatype", "json");
            pairs.append_pair("apikey", &self.token);
            if let Some(interval) = &request.interval {
                pairs.append_pair("interval", interval);
            }
        }
        Ok(url)
    }

    fn transform_data(&self, data: String, request: &Self::Request) -> MarketResult<MarketSeries> {
        // AlphaVantage returns dynamic keys for time series data depending on the interval/function.
        // We use a generic approach with serde_json::Value or multiple structs.

        let mut data_series: Vec<Series> = Vec::new();
        let v: serde_json::Value = serde_json::from_str(&data)?;

        // Find the key that contains "Time Series"
        let series_key = v
            .as_object()
            .and_then(|obj| obj.keys().find(|k| k.contains("Time Series")))
            .ok_or_else(|| {
                MarketError::DownloadedData("Time Series data not found in response".to_string())
            })?;

        let series_data = v
            .get(series_key)
            .and_then(|s| s.as_object())
            .ok_or_else(|| {
                MarketError::DownloadedData("Invalid Time Series data format".to_string())
            })?;

        for (date_str, values) in series_data {
            let open: f32 = values
                .get("1. open")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| MarketError::ParsingError("Unable to parse Open".to_string()))?;
            let high: f32 = values
                .get("2. high")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| MarketError::ParsingError("Unable to parse High".to_string()))?;
            let low: f32 = values
                .get("3. low")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| MarketError::ParsingError("Unable to parse Low".to_string()))?;
            let close: f32 = values
                .get("4. close")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| MarketError::ParsingError("Unable to parse Close".to_string()))?;
            let volume: f32 = values
                .get("5. volume")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| MarketError::ParsingError("Unable to parse Volume".to_string()))?;

            let date = if date_str.len() > 10 {
                NaiveDate::parse_from_str(&date_str[..10], "%Y-%m-%d")
                    .map_err(|e| MarketError::ParsingError(e.to_string()))?
            } else {
                NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                    .map_err(|e| MarketError::ParsingError(e.to_string()))?
            };

            data_series.push(Series {
                date,
                open,
                close,
                high,
                low,
                volume,
            });
        }

        data_series.sort_by_key(|item| item.date);

        Ok(MarketSeries {
            symbol: request.symbol.clone(),
            interval: match request.function {
                Function::Intraday => request
                    .interval
                    .as_ref()
                    .map(|i| i.clone().into())
                    .unwrap_or(Interval::Daily),
                Function::Daily => Interval::Daily,
                Function::Weekly => Interval::Weekly,
                Function::Monthly => Interval::Monthly,
            },
            data: data_series,
        })
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Function::Intraday => "TIME_SERIES_INTRADAY",
            Function::Daily => "TIME_SERIES_DAILY",
            Function::Weekly => "TIME_SERIES_WEEKLY",
            Function::Monthly => "TIME_SERIES_MONTHLY",
        };
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for OutputSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            OutputSize::Compact => "Compact",
            OutputSize::Full => "Full",
        };
        write!(f, "{}", s)
    }
}
